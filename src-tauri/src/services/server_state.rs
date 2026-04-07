//! Server state management for tracking running ARK server processes.
//!
//! Maintains a global HashMap of ProfileName -> ServerHandle for currently
//! running servers and provides state transition management.

use crate::services::rcon_client::{parse_player_list, ArkRconClient, PlayerInfo, RconConnError};
use chrono::{DateTime, Utc};
use encoding_rs::WINDOWS_1252;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::Duration;
use sysinfo::System;
use tauri::Emitter;
use tokio::sync::Mutex;
use tracing::{debug, error, warn};

/// Handle to a running ARK server process.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerHandle {
    /// Process ID of the running server.
    pub pid: u32,
    /// Name of the profile that started this server.
    pub profile_name: String,
    /// When the server was started.
    pub started_at: DateTime<Utc>,
    /// Path to the ShooterGameServer.exe being used.
    pub ark_exe_path: PathBuf,
    /// Query port for the server (used for status detection).
    pub port: u16,
}

/// Current status of a server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServerStatus {
    /// Server is not running.
    Stopped,
    /// Server is in the process of starting.
    Starting,
    /// Server is running normally.
    Running,
    /// Server is in the process of stopping.
    Stopping,
    /// Server has crashed or exited unexpectedly.
    Crashed,
}

/// Errors that can occur when starting a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StartError {
    /// Profile not found.
    ProfileNotFound { name: String },
    /// Failed to load profile JSON.
    ProfileLoadFailed { name: String, reason: String },
    /// Server installation not found or invalid.
    InstallNotFound { guidance: String },
    /// Failed to spawn the server process.
    SpawnFailed { reason: String },
    /// Server for this profile is already running.
    AlreadyRunning { name: String, pid: u32 },
}

impl std::fmt::Display for StartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartError::ProfileNotFound { name } => {
                write!(f, "Profile '{}' not found", name)
            }
            StartError::ProfileLoadFailed { name, reason } => {
                write!(f, "Failed to load profile '{}': {}", name, reason)
            }
            StartError::InstallNotFound { guidance } => {
                write!(f, "Server installation not found. {}", guidance)
            }
            StartError::SpawnFailed { reason } => {
                write!(f, "Failed to spawn server process: {}", reason)
            }
            StartError::AlreadyRunning { name, pid } => {
                write!(
                    f,
                    "Server for profile '{}' is already running (PID: {})",
                    name, pid
                )
            }
        }
    }
}

impl std::error::Error for StartError {}

/// Errors that can occur when stopping a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StopError {
    /// Server for this profile is not running.
    NotRunning { name: String },
    /// Failed to send graceful stop command.
    GracefulStopFailed { name: String, reason: String },
    /// Failed to kill the server process.
    KillFailed { name: String, reason: String },
    /// Timeout while waiting for server to stop.
    Timeout { name: String },
}

impl std::fmt::Display for StopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StopError::NotRunning { name } => {
                write!(f, "No server running for profile '{}'", name)
            }
            StopError::GracefulStopFailed { name, reason } => {
                write!(f, "Failed to gracefully stop server '{}': {}", name, reason)
            }
            StopError::KillFailed { name, reason } => {
                write!(f, "Failed to kill server '{}': {}", name, reason)
            }
            StopError::Timeout { name } => {
                write!(f, "Timeout waiting for server '{}' to stop", name)
            }
        }
    }
}

impl std::error::Error for StopError {}

/// A single line of console output from an ARK server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleLine {
    /// Name of the profile this line belongs to.
    pub profile_name: String,
    /// Timestamp when this line was received.
    pub timestamp: DateTime<Utc>,
    /// The actual text content of the line.
    pub line: String,
    /// Source of the output: "stdout" or "stderr".
    pub source: String,
}

/// ANSI escape code stripping regex.
/// Matches common ANSI escape sequences including colors and cursor movement.
static ANSI_ESCAPE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\x1b\x9b)|\x1b[()#;?]*[0-9A-ORZcf-nqry=>]").unwrap());

/// Strips ANSI escape codes from a string.
pub fn strip_ansi(input: &str) -> String {
    ANSI_ESCAPE_REGEX.replace_all(input, "").to_string()
}

/// Decodes bytes from ARK server output, handling UTF-8 and Windows-1252 encoding.
/// Returns a lossy string conversion.
pub fn decode_console_output(bytes: &[u8]) -> String {
    // First try UTF-8
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }

    // Fall back to Windows-1252, lossy
    let (decoded, _, _) = WINDOWS_1252.decode(bytes);
    decoded.into_owned()
}

/// Maximum number of lines to keep in the console buffer per profile.
const MAX_CONSOLE_BUFFER_LINES: usize = 1000;

/// Global console buffer state tracking console output for all profiles.
/// Uses a HashMap keyed by profile name, with VecDeque values for rolling buffer.
pub struct ConsoleBuffer {
    /// Map of profile name to deque of console lines.
    buffers: HashMap<String, VecDeque<ConsoleLine>>,
}

impl ConsoleBuffer {
    /// Creates a new empty ConsoleBuffer.
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
        }
    }

    /// Gets the console buffer for a profile, creating it if it doesn't exist.
    fn get_or_create_buffer(&mut self, profile_name: &str) -> &mut VecDeque<ConsoleLine> {
        self.buffers.entry(profile_name.to_string()).or_default()
    }

    /// Adds a line to the console buffer for a profile.
    /// Implements rolling buffer - removes oldest line when capacity is exceeded.
    pub fn push_line(&mut self, profile_name: &str, line: ConsoleLine) {
        let buffer = self.get_or_create_buffer(profile_name);

        // Remove oldest line if at capacity
        if buffer.len() >= MAX_CONSOLE_BUFFER_LINES {
            buffer.pop_front();
        }

        buffer.push_back(line);
    }

    /// Gets all buffered console lines for a profile.
    pub fn get_lines(&self, profile_name: &str) -> Vec<ConsoleLine> {
        self.buffers
            .get(profile_name)
            .map(|buffer| buffer.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Clears the console buffer for a profile.
    pub fn clear(&mut self, profile_name: &str) {
        self.buffers.remove(profile_name);
    }

    /// Removes a profile's buffer entirely.
    pub fn remove(&mut self, profile_name: &str) {
        self.buffers.remove(profile_name);
    }
}

impl Default for ConsoleBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Lazy-initialized global console buffer wrapped in a Mutex for thread-safe access.
pub static CONSOLE_BUFFER: std::sync::LazyLock<Mutex<ConsoleBuffer>> =
    std::sync::LazyLock::new(|| Mutex::new(ConsoleBuffer::new()));

/// Returns the logs directory path: %APPDATA%\ArkServerManager\logs
pub fn logs_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ArkServerManager")
        .join("logs")
}

/// Returns the log file path for a profile and timestamp.
/// Path: %APPDATA%\ArkServerManager\logs\{profile_name}\{timestamp}.log
pub fn get_log_file_path(profile_name: &str, timestamp: &DateTime<Utc>) -> PathBuf {
    let log_dir = logs_dir().join(profile_name);
    let filename = format!("{}.log", timestamp.format("%Y-%m-%d_%H-%M-%S"));
    log_dir.join(filename)
}

/// Checks if the ARK server process is running for the given profile.
///
/// Uses the `sysinfo` crate to enumerate processes and check if
/// `ShooterGameServer.exe` with the matching PID exists.
pub fn check_process_status(profile_name: &str) -> bool {
    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    // Find our process by looking for ShooterGameServer.exe with matching profile
    for (pid, process) in sys.processes() {
        let exe_name = process.name().to_string_lossy();
        if exe_name.contains("ShooterGameServer") {
            // On Windows, the process name might be "ShooterGameServer.exe"
            // We can't directly get the command line args from sysinfo easily,
            // so we check if the PID matches what we have stored
            if let Some(handle) = SERVER_STATE.blocking_lock().get_handle(profile_name) {
                if pid.as_u32() == handle.pid {
                    debug!(
                        "Process check: profile '{}' found with PID {}",
                        profile_name, handle.pid
                    );
                    return true;
                }
            }
        }
    }

    debug!(
        "Process check: no running process found for profile '{}'",
        profile_name
    );
    false
}

/// Checks if the ARK server port is accepting connections.
///
/// Uses TCP connect to `127.0.0.1:{port}` with a 500ms timeout.
pub fn check_port_status(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    match TcpStream::connect_timeout(
        &addr.parse().unwrap_or_else(|_| {
            warn!("Invalid port address: {}", port);
            std::net::SocketAddr::from(([127, 0, 0, 1], port))
        }),
        Duration::from_millis(500),
    ) {
        Ok(_stream) => {
            debug!("Port check: port {} is open", port);
            true
        }
        Err(e) => {
            debug!("Port check: port {} is not reachable: {}", port, e);
            false
        }
    }
}

/// Status change event payload for Tauri events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusChangedEvent {
    pub profile_name: String,
    pub old_status: ServerStatus,
    pub new_status: ServerStatus,
}

/// Global server state tracking running ARK servers.
///
/// Uses a HashMap keyed by profile name, with ServerHandle values.
pub struct ServerState {
    /// Map of profile name to server handle for running servers.
    servers: HashMap<String, ServerHandle>,
    /// Map of profile name to current status.
    statuses: HashMap<String, ServerStatus>,
    /// Map of profile name to previous status (for transition detection).
    previous_statuses: HashMap<String, ServerStatus>,
    /// Map of profile name to last retrieved player list.
    player_lists: HashMap<String, Vec<PlayerInfo>>,
}

impl ServerState {
    /// Creates a new empty ServerState.
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
            statuses: HashMap::new(),
            previous_statuses: HashMap::new(),
            player_lists: HashMap::new(),
        }
    }

    /// Checks if a server is currently running for the given profile.
    pub fn is_running(&self, profile_name: &str) -> bool {
        self.servers.contains_key(profile_name)
    }

    /// Gets the current status for a profile.
    pub fn get_status(&self, profile_name: &str) -> ServerStatus {
        self.statuses
            .get(profile_name)
            .cloned()
            .unwrap_or(ServerStatus::Stopped)
    }

    /// Gets the previous status for a profile.
    pub fn get_previous_status(&self, profile_name: &str) -> Option<ServerStatus> {
        self.previous_statuses.get(profile_name).cloned()
    }

    /// Gets the player list for a profile.
    pub fn get_player_list(&self, profile_name: &str) -> Option<&Vec<PlayerInfo>> {
        self.player_lists.get(profile_name)
    }

    /// Sets the player list for a profile.
    pub fn set_player_list(&mut self, profile_name: &str, players: Vec<PlayerInfo>) {
        debug!(
            "Setting player list for profile '{}': {} players",
            profile_name,
            players.len()
        );
        self.player_lists.insert(profile_name.to_string(), players);
    }

    /// Gets the handle for a running server.
    pub fn get_handle(&self, profile_name: &str) -> Option<&ServerHandle> {
        self.servers.get(profile_name)
    }

    /// Inserts a new server handle into the state.
    pub fn insert(&mut self, profile_name: String, handle: ServerHandle) {
        debug!("Inserting server handle for profile '{}'", profile_name);
        self.servers.insert(profile_name.clone(), handle);
        self.statuses
            .insert(profile_name.clone(), ServerStatus::Running);
        self.previous_statuses
            .insert(profile_name, ServerStatus::Running);
    }

    /// Updates the status for a profile, tracking the previous status.
    /// Returns the previous status if there was a transition.
    /// Updates the status for a profile, tracking the previous status.
    /// Returns the previous status if there was a transition, or None if no change occurred.
    pub fn set_status(&mut self, profile_name: &str, status: ServerStatus) -> Option<ServerStatus> {
        let old_status = self.statuses.get(profile_name).cloned();

        // Only track transition if status is actually changing
        if old_status.as_ref() != Some(&status) {
            debug!(
                "Setting server status for profile '{}' from {:?} to {:?}",
                profile_name, old_status, status
            );
            // Store current as previous before updating
            if let Some(ref current) = old_status {
                self.previous_statuses
                    .insert(profile_name.to_string(), current.clone());
            }
            self.statuses.insert(profile_name.to_string(), status);
            old_status // Return the old status to indicate a transition occurred
        } else {
            None // No change occurred
        }
    }

    /// Removes a server from the state.
    pub fn remove(&mut self, profile_name: &str) {
        debug!("Removing server state for profile '{}'", profile_name);
        self.servers.remove(profile_name);
        self.statuses.remove(profile_name);
        self.previous_statuses.remove(profile_name);
    }

    /// Gets all running server handles.
    pub fn all_handles(&self) -> &HashMap<String, ServerHandle> {
        &self.servers
    }

    /// Detects the current status based on process and port checks.
    ///
    /// - If handle exists but process dead → `Crashed`
    /// - If handle exists and process alive → `Running`
    /// - If port check succeeds but no handle → `Running` (recovered case)
    /// - If port fails and no handle → `Stopped`
    ///
    /// Returns the detected status and whether it changed from the previous status.
    pub fn detect_status(&self, profile_name: &str, port: u16) -> (ServerStatus, bool) {
        let handle_exists = self.servers.contains_key(profile_name);
        let process_alive = check_process_status(profile_name);
        let port_open = check_port_status(port);

        let current_status = self.get_status(profile_name);

        let new_status = if handle_exists && !process_alive {
            // Handle exists but process is dead - crashed
            ServerStatus::Crashed
        } else if handle_exists && process_alive {
            // Handle exists and process is alive - running
            ServerStatus::Running
        } else if port_open && !handle_exists {
            // Port is open but no handle - recovered (server started externally)
            ServerStatus::Running
        } else {
            // No handle, no process, port closed - stopped
            ServerStatus::Stopped
        };

        let changed = current_status != new_status;

        if changed {
            debug!(
                "Status detect for '{}': port={}, handle_exists={}, process_alive={}, \
                 port_open={} -> status changed from {:?} to {:?}",
                profile_name,
                port,
                handle_exists,
                process_alive,
                port_open,
                current_status,
                new_status
            );
        }

        (new_status, changed)
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Lazy-initialized global server state wrapped in a Mutex for thread-safe access.
pub static SERVER_STATE: std::sync::LazyLock<Mutex<ServerState>> =
    std::sync::LazyLock::new(|| Mutex::new(ServerState::new()));

/// Player list update event payload for Tauri events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerListUpdatedEvent {
    pub profile_name: String,
    pub players: Vec<PlayerInfo>,
}

/// Starts the background status polling task.
///
/// This task runs every 2 seconds and checks the status of all running servers.
/// When a status change is detected, it emits a `status-changed` Tauri event.
///
/// Additionally, it polls player lists every 10 seconds via RCON and emits
/// `player-list-updated` events.
pub fn start_status_polling(app: tauri::AppHandle) {
    let app_handle = app.clone();

    std::thread::spawn(move || {
        tracing::info!("Starting background status polling task");

        // Create a Tokio runtime for async RCON operations
        let runtime = tokio::runtime::Runtime::new()
            .expect("Failed to create Tokio runtime for player list polling");

        let mut player_poll_counter: u32 = 0;

        loop {
            std::thread::sleep(std::time::Duration::from_secs(2));

            let profile_names: Vec<String> = {
                let state = SERVER_STATE.blocking_lock();
                state.all_handles().keys().cloned().collect()
            };

            for profile_name in profile_names {
                let (new_status, changed) = {
                    let state = SERVER_STATE.blocking_lock();
                    let port = state.get_handle(&profile_name).map(|h| h.port).unwrap_or(0);
                    state.detect_status(&profile_name, port)
                };

                if changed {
                    let old_status = {
                        let mut state = SERVER_STATE.blocking_lock();
                        state.set_status(&profile_name, new_status.clone())
                    };

                    if let Some(old) = old_status {
                        tracing::info!(
                            "Status change detected for '{}': {:?} -> {:?}",
                            profile_name,
                            old,
                            new_status
                        );

                        let event = StatusChangedEvent {
                            profile_name: profile_name.clone(),
                            old_status: old,
                            new_status: new_status.clone(),
                        };

                        // Emit the status-changed event
                        let _ = app_handle.emit("status-changed", &event);

                        // If crashed, emit a specific crash notification event
                        if new_status == ServerStatus::Crashed {
                            tracing::warn!("Server '{}' has crashed!", profile_name);
                            let _ = app_handle.emit("server-crashed", &profile_name);
                        }
                    }
                }

                // Poll player list every 10 seconds (every 5 iterations at 2s intervals)
                player_poll_counter = player_poll_counter.wrapping_add(1);
                if player_poll_counter.is_multiple_of(5) {
                    poll_player_list(&app_handle, &runtime, &profile_name);
                }
            }
        }
    });
}

/// Polls the player list for a profile via RCON and emits the result.
///
/// This function connects to the ARK server RCON port, sends the PlayerId command,
/// parses the response, stores it in server state, and emits a Tauri event.
/// Connection failures are handled gracefully since the server may not be fully started.
fn poll_player_list(
    app_handle: &tauri::AppHandle,
    runtime: &tokio::runtime::Runtime,
    profile_name: &str,
) {
    // Load the profile to get admin_password and port
    let (rcon_port, admin_password) = match load_profile(profile_name) {
        Ok(profile) => {
            // RCON port is QueryPort + 1 (ARK convention)
            let rcon_port = (profile.port as u16).saturating_add(1);
            let password = profile.admin_password.unwrap_or_default();
            (rcon_port, password)
        }
        Err(e) => {
            tracing::debug!(
                "Could not load profile '{}' for player list polling: {}",
                profile_name,
                e
            );
            return;
        }
    };

    // Build the RCON address
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], rcon_port));

    // Run the async RCON operations in the runtime
    let result: Result<Vec<PlayerInfo>, RconConnError> = runtime.block_on(async {
        let client = match ArkRconClient::new(profile_name.to_string(), addr, &admin_password).await
        {
            Ok(client) => client,
            Err(e) => {
                tracing::debug!("RCON connection failed for '{}': {}", profile_name, e);
                return Err(e);
            }
        };

        match client.execute("PlayerId").await {
            Ok(output) => Ok(parse_player_list(&output)),
            Err(e) => {
                tracing::debug!("RCON command failed for '{}': {}", profile_name, e);
                Err(e)
            }
        }
    });

    match result {
        Ok(players) => {
            // Store in server state
            {
                let mut state = SERVER_STATE.blocking_lock();
                state.set_player_list(profile_name, players.clone());
            }

            // Emit the player-list-updated event
            let event = PlayerListUpdatedEvent {
                profile_name: profile_name.to_string(),
                players,
            };
            let _ = app_handle.emit("player-list-updated", &event);
            tracing::debug!("Emitted player-list-updated for profile '{}'", profile_name);
        }
        Err(e) => {
            tracing::debug!("Player list poll failed for '{}': {}", profile_name, e);
        }
    }
}

pub fn profiles_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ArkServerManager")
        .join("profiles")
}

/// Builds the ARK server command-line arguments from a profile.
///
/// Format: `ShooterGameServer.exe {map}?QueryPort={port}?MaxPlayers={max}...`
///
/// # Arguments
///
/// * `profile` - The profile to build arguments for
///
/// # Returns
///
/// A vector of command-line arguments
pub fn build_server_args(profile: &crate::models::Profile) -> Vec<String> {
    let mut args = Vec::new();

    // The map name
    args.push(profile.map.clone());

    // Query port
    args.push(format!("?QueryPort={}", profile.port));

    // Max players
    args.push(format!("?MaxPlayers={}", profile.max_players));

    // Difficulty (if not default)
    if profile.difficulty != 1.0 {
        args.push(format!("?Difficulty={}", profile.difficulty));
    }

    // Server name (from profile name) and passwords
    args.push(format!(
        "?ServerPassword={}?ServerAdminPassword={}",
        profile.name,
        profile.admin_password.as_deref().unwrap_or("")
    ));
    // Note: ARK uses ServerPassword for rcon password and ServerAdminPassword for admin

    // Extra settings from the profile
    for (key, value) in &profile.extra_settings {
        args.push(format!("?{}={}", key, value));
    }

    for (key, value) in &profile.extra_user_settings {
        args.push(format!("?{}={}", key, value));
    }

    args
}

/// Gets the working directory for ARK server execution.
///
/// This is the parent of Win64: ShooterGame\Binaries\Win64\ -> ShooterGame\Binaries\
pub fn get_working_directory(ark_exe_path: &Path) -> PathBuf {
    ark_exe_path
        .parent() // Win64
        .and_then(|p| p.parent()) // Binaries
        .map(PathBuf::from)
        .unwrap_or_else(|| ark_exe_path.parent().map(PathBuf::from).unwrap_or_default())
}

/// Loads a profile by name from the profiles directory.
pub fn load_profile(name: &str) -> Result<crate::models::Profile, StartError> {
    let path = profiles_dir().join(format!("{}.json", name));

    if !path.exists() {
        return Err(StartError::ProfileNotFound {
            name: name.to_string(),
        });
    }

    let contents = std::fs::read_to_string(&path).map_err(|e| {
        error!("Failed to read profile '{}': {}", name, e);
        StartError::ProfileLoadFailed {
            name: name.to_string(),
            reason: e.to_string(),
        }
    })?;

    serde_json::from_str(&contents).map_err(|e| {
        error!("Failed to parse profile '{}': {}", name, e);
        StartError::ProfileLoadFailed {
            name: name.to_string(),
            reason: e.to_string(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Profile;

    #[test]
    fn test_build_server_args_basic() {
        let profile = Profile {
            name: "TestServer".to_string(),
            map: "TheIsland".to_string(),
            difficulty: 1.0,
            max_players: 70,
            admin_password: None,
            port: 27015,
            ..Default::default()
        };

        let args = build_server_args(&profile);

        assert!(args.contains(&"TheIsland".to_string()));
        assert!(args.contains(&"?QueryPort=27015".to_string()));
        assert!(args.contains(&"?MaxPlayers=70".to_string()));
    }

    #[test]
    fn test_build_server_args_with_difficulty() {
        let profile = Profile {
            name: "HardServer".to_string(),
            map: "ScorchedEarth".to_string(),
            difficulty: 5.0,
            max_players: 10,
            admin_password: Some("secret".to_string()),
            port: 27000,
            ..Default::default()
        };

        let args = build_server_args(&profile);

        assert!(args.contains(&"ScorchedEarth".to_string()));
        assert!(args.contains(&"?Difficulty=5".to_string()));
        assert!(args.contains(&"?MaxPlayers=10".to_string()));
        assert!(args.contains(&"?QueryPort=27000".to_string()));
    }

    #[test]
    fn test_build_server_args_with_extra_settings() {
        let mut extra_settings = HashMap::new();
        extra_settings.insert("ActiveMods".to_string(), "mod1,mod2".to_string());

        let profile = Profile {
            name: "ModServer".to_string(),
            map: "TheIsland".to_string(),
            difficulty: 1.0,
            max_players: 70,
            admin_password: None,
            port: 27015,
            extra_settings,
            ..Default::default()
        };

        let args = build_server_args(&profile);

        assert!(args.contains(&"?ActiveMods=mod1,mod2".to_string()));
    }

    #[test]
    fn test_server_state_insert_get_remove() {
        let mut state = ServerState::new();

        let handle = ServerHandle {
            pid: 1234,
            profile_name: "TestProfile".to_string(),
            started_at: Utc::now(),
            ark_exe_path: PathBuf::from("C:\\ARK\\ShooterGameServer.exe"),
            port: 27015,
        };

        assert!(!state.is_running("TestProfile"));
        assert_eq!(state.get_status("TestProfile"), ServerStatus::Stopped);

        state.insert("TestProfile".to_string(), handle.clone());

        assert!(state.is_running("TestProfile"));
        assert_eq!(state.get_status("TestProfile"), ServerStatus::Running);
        assert_eq!(state.get_handle("TestProfile"), Some(&handle));

        state.remove("TestProfile");

        assert!(!state.is_running("TestProfile"));
        assert_eq!(state.get_status("TestProfile"), ServerStatus::Stopped);
        assert_eq!(state.get_handle("TestProfile"), None);
    }

    #[test]
    fn test_server_state_status_transitions() {
        let mut state = ServerState::new();

        state.set_status("TestProfile", ServerStatus::Starting);
        assert_eq!(state.get_status("TestProfile"), ServerStatus::Starting);

        state.set_status("TestProfile", ServerStatus::Running);
        assert_eq!(state.get_status("TestProfile"), ServerStatus::Running);

        state.set_status("TestProfile", ServerStatus::Stopping);
        assert_eq!(state.get_status("TestProfile"), ServerStatus::Stopping);

        state.set_status("TestProfile", ServerStatus::Stopped);
        assert_eq!(state.get_status("TestProfile"), ServerStatus::Stopped);
    }

    #[cfg(windows)]
    #[test]
    fn test_get_working_directory() {
        let exe_path = PathBuf::from(
            r"C:\Program Files (x86)\Steam\steamapps\common\ARK\ShooterGame\Binaries\Win64\ShooterGameServer.exe",
        );

        let work_dir = get_working_directory(&exe_path);

        assert_eq!(
            work_dir,
            PathBuf::from(
                r"C:\Program Files (x86)\Steam\steamapps\common\ARK\ShooterGame\Binaries"
            )
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn test_get_working_directory() {
        // On non-Windows, test with a Unix-style path
        let exe_path = PathBuf::from(
            "/home/steam/steamapps/common/ARK/ShooterGame/Binaries/Win64/ShooterGameServer.exe",
        );

        let work_dir = get_working_directory(&exe_path);

        assert_eq!(
            work_dir,
            PathBuf::from("/home/steam/steamapps/common/ARK/ShooterGame/Binaries")
        );
    }

    #[test]
    fn test_start_error_display() {
        let err = StartError::ProfileNotFound {
            name: "TestProfile".to_string(),
        };
        assert_eq!(format!("{}", err), "Profile 'TestProfile' not found");

        let err = StartError::AlreadyRunning {
            name: "TestProfile".to_string(),
            pid: 1234,
        };
        assert_eq!(
            format!("{}", err),
            "Server for profile 'TestProfile' is already running (PID: 1234)"
        );
    }

    #[test]
    fn test_stop_error_display() {
        let err = StopError::NotRunning {
            name: "TestProfile".to_string(),
        };
        assert_eq!(
            format!("{}", err),
            "No server running for profile 'TestProfile'"
        );

        let err = StopError::Timeout {
            name: "TestProfile".to_string(),
        };
        assert_eq!(
            format!("{}", err),
            "Timeout waiting for server 'TestProfile' to stop"
        );
    }

    #[test]
    fn test_server_handle_with_port() {
        let handle = ServerHandle {
            pid: 1234,
            profile_name: "TestProfile".to_string(),
            started_at: Utc::now(),
            ark_exe_path: PathBuf::from("C:\\ARK\\ShooterGameServer.exe"),
            port: 27015,
        };

        assert_eq!(handle.pid, 1234);
        assert_eq!(handle.port, 27015);
        assert_eq!(handle.profile_name, "TestProfile");
    }

    #[test]
    fn test_status_changed_event_serialization() {
        let event = StatusChangedEvent {
            profile_name: "TestProfile".to_string(),
            old_status: ServerStatus::Running,
            new_status: ServerStatus::Crashed,
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("TestProfile"));
        assert!(json.contains("Running"));
        assert!(json.contains("Crashed"));
    }

    #[test]
    fn test_server_state_previous_status_tracking() {
        let mut state = ServerState::new();

        // Set initial status
        state.set_status("TestProfile", ServerStatus::Starting);

        // Change status and verify previous is tracked
        let old_status = state.set_status("TestProfile", ServerStatus::Running);
        assert_eq!(old_status, Some(ServerStatus::Starting));
        assert_eq!(
            state.get_previous_status("TestProfile"),
            Some(ServerStatus::Starting)
        );

        // Change again
        let old_status = state.set_status("TestProfile", ServerStatus::Crashed);
        assert_eq!(old_status, Some(ServerStatus::Running));
        assert_eq!(
            state.get_previous_status("TestProfile"),
            Some(ServerStatus::Running)
        );

        // Remove should clear previous status
        state.remove("TestProfile");
        assert_eq!(state.get_previous_status("TestProfile"), None);
    }

    #[test]
    fn test_server_state_insert_initializes_previous_status() {
        let mut state = ServerState::new();

        let handle = ServerHandle {
            pid: 1234,
            profile_name: "TestProfile".to_string(),
            started_at: Utc::now(),
            ark_exe_path: PathBuf::from("C:\\ARK\\ShooterGameServer.exe"),
            port: 27015,
        };

        state.insert("TestProfile".to_string(), handle);

        // After insert, both current and previous should be Running
        assert_eq!(state.get_status("TestProfile"), ServerStatus::Running);
        assert_eq!(
            state.get_previous_status("TestProfile"),
            Some(ServerStatus::Running)
        );
    }

    #[test]
    fn test_server_state_set_status_no_change() {
        let mut state = ServerState::new();

        state.set_status("TestProfile", ServerStatus::Running);
        // Setting the same status should not change anything
        let old_status = state.set_status("TestProfile", ServerStatus::Running);

        // Should return None since status didn't actually change
        assert_eq!(old_status, None);
        assert_eq!(state.get_status("TestProfile"), ServerStatus::Running);
    }
}

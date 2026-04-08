//! Health monitoring module for ARK server processes.
//!
//! Monitors CPU, memory, player count, and uptime metrics for running ARK servers.
//! Emits health-update events every 5 seconds and provides auto-restart functionality
//! when servers crash unexpectedly.

use crate::services::server_state::{ServerStatus, SERVER_STATE};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;
use sysinfo::System;
use tauri::Emitter;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

/// Health metrics for a running ARK server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    /// Name of the profile this health data belongs to.
    pub profile_name: String,
    /// CPU usage percentage (0.0 - 100.0).
    pub cpu_percent: f32,
    /// Memory usage in megabytes.
    pub memory_mb: u64,
    /// Memory usage percentage (0.0 - 100.0).
    pub memory_percent: f32,
    /// Current number of players (from RCON if available).
    pub player_count: u32,
    /// Maximum player count from profile.
    pub max_players: u32,
    /// Server uptime in seconds.
    pub uptime_seconds: u64,
    /// Current server status.
    pub status: ServerStatus,
}

/// Restart attempt tracking for a profile.
#[derive(Debug, Clone)]
struct RestartAttempt {
    /// When the restart was attempted.
    timestamp: DateTime<Utc>,
    /// Whether the restart was successful.
    #[allow(dead_code)]
    success: bool,
}

/// Tracks restart attempts for auto-restart functionality.
struct RestartTracker {
    /// Map of profile name to list of restart attempts in the last 5 minutes.
    attempts: HashMap<String, Vec<RestartAttempt>>,
}

impl RestartTracker {
    fn new() -> Self {
        Self {
            attempts: HashMap::new(),
        }
    }

    /// Records a restart attempt for a profile.
    fn record_attempt(&mut self, profile_name: &str, success: bool) {
        let attempt = RestartAttempt {
            timestamp: Utc::now(),
            success,
        };
        self.attempts
            .entry(profile_name.to_string())
            .or_default()
            .push(attempt);
    }

    /// Gets the number of restart attempts for a profile in the last 5 minutes.
    fn get_attempt_count(&self, profile_name: &str) -> u32 {
        let five_minutes_ago = Utc::now() - chrono::Duration::minutes(5);
        self.attempts
            .get(profile_name)
            .map(|attempts| {
                attempts
                    .iter()
                    .filter(|a| a.timestamp > five_minutes_ago)
                    .count() as u32
            })
            .unwrap_or(0)
    }

    /// Checks if a restart should be allowed based on max attempts.
    fn should_allow_restart(&self, profile_name: &str, max_attempts: u32) -> bool {
        self.get_attempt_count(profile_name) < max_attempts
    }

    /// Cleans up old restart attempts (older than 5 minutes).
    fn cleanup_old_attempts(&mut self) {
        let five_minutes_ago = Utc::now() - chrono::Duration::minutes(5);
        for attempts in self.attempts.values_mut() {
            attempts.retain(|a| a.timestamp > five_minutes_ago);
        }
        // Remove empty vectors
        self.attempts.retain(|_, v| !v.is_empty());
    }

    /// Resets restart attempts for a profile after successful running period.
    fn reset_if_stable(&mut self, profile_name: &str, stable_duration_secs: u64) {
        if let Some(attempts) = self.attempts.get_mut(profile_name) {
            let cutoff = Utc::now() - chrono::Duration::seconds(stable_duration_secs as i64);
            attempts.retain(|a| a.timestamp > cutoff);
        }
    }
}

/// Global restart tracker wrapped in a Mutex for thread-safe access.
static RESTART_TRACKER: LazyLock<Mutex<RestartTracker>> =
    LazyLock::new(|| Mutex::new(RestartTracker::new()));

/// Time after which restart attempts are reset (5 minutes of stable running).
const STABLE_DURATION_SECS: u64 = 300;

/// Gets the ARK server process metrics using sysinfo.
///
/// Returns CPU percentage, memory in bytes, and whether the process was found.
pub fn get_ark_process_metrics() -> (f32, u64, bool) {
    let mut sys = System::new();
    // Need to refresh all processes to find ARK server
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let mut cpu_percent = 0.0_f32;
    let mut memory_bytes = 0_u64;
    let mut found = false;

    for process in sys.processes().values() {
        let exe_name = process.name().to_string_lossy();
        if exe_name.contains("ShooterGameServer") {
            found = true;
            cpu_percent = process.cpu_usage();
            memory_bytes = process.memory();
            debug!(
                "Found ARK process: {} (CPU: {:.1}%, Memory: {} bytes)",
                exe_name, cpu_percent, memory_bytes
            );
            break; // Assuming only one ARK server process
        }
    }

    (cpu_percent, memory_bytes, found)
}

/// Calculates memory percentage given used bytes and total system bytes.
pub fn calculate_memory_percent(used_bytes: u64, total_bytes: u64) -> f32 {
    if total_bytes == 0 {
        return 0.0;
    }
    ((used_bytes as f64 / total_bytes as f64) * 100.0) as f32
}

/// Gets the total system memory in bytes.
pub fn get_total_memory_bytes() -> u64 {
    let sys = System::new();
    sys.total_memory()
}

/// Calculates uptime in seconds from a started_at timestamp.
pub fn calculate_uptime_seconds(started_at: &DateTime<Utc>) -> u64 {
    let now = Utc::now();
    let duration = now.signed_duration_since(*started_at);
    duration.num_seconds().max(0) as u64
}

/// Gets health metrics for a specific profile.
///
/// Returns None if the profile is not running.
pub fn get_health_metrics(profile_name: &str) -> Option<HealthMetrics> {
    let state = SERVER_STATE.blocking_lock();
    let handle = state.get_handle(profile_name)?;
    let status = state.get_status(profile_name);
    let player_list = state.get_player_list(profile_name);

    // Get process metrics
    let (cpu_percent, memory_bytes, process_found) = get_ark_process_metrics();

    // Calculate memory in MB and percentage
    let memory_mb = memory_bytes / (1024 * 1024);
    let total_memory = get_total_memory_bytes();
    let memory_percent = calculate_memory_percent(memory_bytes, total_memory);

    // Get player count
    let player_count = player_list.map(|p| p.len() as u32).unwrap_or(0);

    // Get max players from handle (we need to load the profile for this)
    let max_players = {
        // Try to get from server state or default
        // The actual max_players comes from profile, which we'd need to load
        // For now, use a default that will be updated by the frontend
        70 // This will be overridden when actual profile is loaded
    };

    // Calculate uptime
    let uptime_seconds = if process_found {
        calculate_uptime_seconds(&handle.started_at)
    } else {
        0
    };

    Some(HealthMetrics {
        profile_name: profile_name.to_string(),
        cpu_percent,
        memory_mb,
        memory_percent,
        player_count,
        max_players,
        uptime_seconds,
        status,
    })
}

/// Starts the health monitoring background task.
///
/// This task runs every 5 seconds and emits health-update events for all running servers.
pub fn start_health_monitoring(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        info!("Starting health monitoring background task");

        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));

            let profile_names: Vec<String> = {
                let state = SERVER_STATE.blocking_lock();
                state.all_handles().keys().cloned().collect()
            };

            // Cleanup old restart attempts periodically
            {
                let mut tracker = RESTART_TRACKER.blocking_lock();
                tracker.cleanup_old_attempts();
            }

            for profile_name in profile_names {
                // Check if server has been stable for > 5 minutes to reset restart counter
                {
                    let state = SERVER_STATE.blocking_lock();
                    if let Some(handle) = state.get_handle(&profile_name) {
                        let uptime = calculate_uptime_seconds(&handle.started_at);
                        if uptime > STABLE_DURATION_SECS {
                            let mut tracker = RESTART_TRACKER.blocking_lock();
                            tracker.reset_if_stable(&profile_name, STABLE_DURATION_SECS);
                        }
                    }
                }

                // Get and emit health metrics
                if let Some(metrics) = get_health_metrics(&profile_name) {
                    if let Err(e) = app.emit("health-update", &metrics) {
                        error!(
                            "Failed to emit health-update for profile '{}': {}",
                            profile_name, e
                        );
                    } else {
                        debug!(
                            "Emitted health-update for '{}': CPU={:.1}%, Memory={}MB ({:.1}%)",
                            profile_name,
                            metrics.cpu_percent,
                            metrics.memory_mb,
                            metrics.memory_percent
                        );
                    }
                }
            }
        }
    });
}

/// Handles auto-restart logic after a crash is detected.
///
/// Returns true if a restart was initiated, false if max attempts reached.
pub async fn handle_auto_restart(
    app: &tauri::AppHandle,
    profile_name: &str,
    auto_restart_on_crash: bool,
    auto_restart_delay_secs: u32,
    max_restart_attempts: u32,
) -> bool {
    if !auto_restart_on_crash {
        info!(
            "Auto-restart disabled for profile '{}', not restarting",
            profile_name
        );
        return false;
    }

    let tracker_ref = &*RESTART_TRACKER.lock().await;
    if !tracker_ref.should_allow_restart(profile_name, max_restart_attempts) {
        warn!(
            "Max restart attempts ({}) reached for profile '{}'. Not restarting.",
            max_restart_attempts, profile_name
        );

        // Emit auto-restart-exhausted event
        if let Err(e) = app.emit("auto-restart-exhausted", profile_name) {
            error!(
                "Failed to emit auto-restart-exhausted for profile '{}': {}",
                profile_name, e
            );
        }

        return false;
    }

    // Wait the configured delay
    info!(
        "Waiting {} seconds before restarting profile '{}'",
        auto_restart_delay_secs, profile_name
    );
    tokio::time::sleep(tokio::time::Duration::from_secs(
        auto_restart_delay_secs as u64,
    ))
    .await;

    // Record the restart attempt
    {
        let mut tracker = RESTART_TRACKER.lock().await;
        tracker.record_attempt(profile_name, false); // Will be marked success if server starts
    }

    // Emit request to restart the server
    // The actual restart will be handled by the server_state module
    if let Err(e) = app.emit("request-server-start", profile_name) {
        error!(
            "Failed to emit request-server-start for profile '{}': {}",
            profile_name, e
        );
        return false;
    }

    info!("Auto-restart initiated for profile '{}'", profile_name);
    true
}

/// Marks a restart attempt as successful (called when server starts running).
pub async fn mark_restart_success(profile_name: &str) {
    let mut tracker = RESTART_TRACKER.lock().await;
    tracker.record_attempt(profile_name, true);
    info!(
        "Restart attempt marked as successful for profile '{}'",
        profile_name
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_memory_percent() {
        assert_eq!(calculate_memory_percent(0, 1000), 0.0);
        assert_eq!(calculate_memory_percent(500, 1000), 50.0);
        assert_eq!(calculate_memory_percent(1000, 1000), 100.0);
        assert_eq!(calculate_memory_percent(100, 0), 0.0); // Avoid division by zero
    }

    #[test]
    fn test_restart_tracker_new() {
        let tracker = RestartTracker::new();
        assert_eq!(tracker.get_attempt_count("test"), 0);
    }

    #[test]
    fn test_restart_tracker_record_attempt() {
        let mut tracker = RestartTracker::new();
        tracker.record_attempt("test", true);
        assert_eq!(tracker.get_attempt_count("test"), 1);
        tracker.record_attempt("test", false);
        assert_eq!(tracker.get_attempt_count("test"), 2);
    }

    #[test]
    fn test_restart_tracker_should_allow() {
        let mut tracker = RestartTracker::new();
        assert!(tracker.should_allow_restart("test", 3));
        tracker.record_attempt("test", false);
        assert!(tracker.should_allow_restart("test", 3));
        tracker.record_attempt("test", false);
        tracker.record_attempt("test", false);
        assert!(!tracker.should_allow_restart("test", 3));
        // But should allow for different profile
        assert!(tracker.should_allow_restart("other", 3));
    }

    #[tokio::test]
    async fn test_mark_restart_success() {
        mark_restart_success("test_profile").await;
        let tracker = RESTART_TRACKER.lock().await;
        assert_eq!(tracker.get_attempt_count("test_profile"), 1);
    }
}

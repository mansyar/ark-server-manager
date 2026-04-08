//! Server lifecycle Tauri commands (start, stop, status, console buffer).

use crate::services::server_state::{
    build_server_args, decode_console_output, get_log_file_path, get_working_directory,
    load_profile as load_profile_from_state, strip_ansi, ConsoleLine, ServerHandle, ServerStatus,
    CONSOLE_BUFFER, SERVER_STATE,
};
use crate::services::{notify_server_started, notify_server_stopped};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

/// Starts an ARK server for the given profile.
///
/// Loads the profile, resolves the server installation, builds command-line
/// arguments, spawns the ShooterGameServer process, and tracks it in the global
/// server state. Also spawns async tasks to read stdout/stderr and emit console events.
#[tauri::command]
pub async fn start_server(profile_name: String, app: AppHandle) -> Result<ServerHandle, String> {
    info!("start_server called for profile: {}", profile_name);

    // Check if already running
    {
        let state = SERVER_STATE.lock().await;
        if state.is_running(&profile_name) {
            let handle = state.get_handle(&profile_name);
            if let Some(h) = handle {
                return Err(format!(
                    "Server for profile '{}' is already running (PID: {})",
                    profile_name, h.pid
                ));
            }
        }
    }

    // Load the profile
    let profile = load_profile_from_state(&profile_name).map_err(|e| e.to_string())?;

    // Resolve the ARK executable path
    let ark_exe_path =
        crate::services::server_discovery::resolve_ark_exe(&profile).map_err(|e| e.to_string())?;

    // Build command-line arguments
    let args = build_server_args(&profile);

    // Get working directory (parent of Win64)
    let work_dir = get_working_directory(&ark_exe_path);

    debug!(
        "Starting ARK server: exe={:?}, work_dir={:?}, args={:?}",
        ark_exe_path, work_dir, args
    );

    // Spawn the process using tokio::process::Command for async stdout/stderr
    let mut child = tokio::process::Command::new(&ark_exe_path)
        .args(&args)
        .current_dir(&work_dir)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            error!("Failed to spawn ARK server process: {}", e);
            format!("Failed to spawn server process: {}", e)
        })?;

    let pid = child.id().ok_or_else(|| {
        error!("Failed to get PID of spawned process");
        "Failed to get PID of spawned process".to_string()
    })?;
    let started_at = chrono::Utc::now();

    info!(
        "ARK server started: profile='{}', PID={}, started_at={}",
        profile_name, pid, started_at
    );

    // Create handle and store in global state
    let handle = ServerHandle {
        pid,
        profile_name: profile_name.clone(),
        started_at,
        ark_exe_path: ark_exe_path.clone(),
        port: profile.port as u16,
    };

    {
        let mut state = SERVER_STATE.lock().await;
        state.insert(profile_name.clone(), handle.clone());
    }

    // Set up log file for this server session
    let log_path = get_log_file_path(&profile_name, &started_at);
    let log_dir = log_path.parent().unwrap_or(&log_path);

    // Create log directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(log_dir) {
        error!("Failed to create log directory {:?}: {}", log_dir, e);
    }

    // Open log file for appending
    let log_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| format!("Failed to open log file {:?}: {}", log_path, e))?,
    ));

    // Spawn async tasks to read stdout and stderr
    let profile_name_stdout = profile_name.clone();
    let profile_name_stderr = profile_name.clone();
    let app_stdout = app.clone();
    let app_stderr = app.clone();
    let log_file_stdout = Arc::clone(&log_file);
    let log_file_stderr = Arc::clone(&log_file);

    // Get stdout and stderr streams
    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    // Spawn task to read stdout
    let _stdout_handle = tokio::spawn(async move {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let timestamp = chrono::Utc::now();
                    let raw_line = decode_console_output(line.as_bytes());
                    let clean_line = strip_ansi(&raw_line);

                    let console_line = ConsoleLine {
                        profile_name: profile_name_stdout.clone(),
                        timestamp,
                        line: clean_line.clone(),
                        source: "stdout".to_string(),
                    };

                    // Store in buffer
                    {
                        let mut buffer = CONSOLE_BUFFER.lock().await;
                        buffer.push_line(&profile_name_stdout, console_line.clone());
                    }

                    // Write to log file
                    {
                        let mut log = log_file_stdout.lock().await;
                        let log_entry = format!(
                            "[{}] [stdout] {}\n",
                            timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
                            clean_line
                        );
                        let _ = log.write_all(log_entry.as_bytes());
                    }

                    // Emit event
                    let _ = app_stdout.emit("console-output", &console_line);
                }
                Err(e) => {
                    error!("Error reading stdout: {}", e);
                    break;
                }
            }
        }
    });

    // Spawn task to read stderr
    let _stderr_handle = tokio::spawn(async move {
        let mut reader = BufReader::new(stderr);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let timestamp = chrono::Utc::now();
                    let raw_line = decode_console_output(line.as_bytes());
                    let clean_line = strip_ansi(&raw_line);

                    let console_line = ConsoleLine {
                        profile_name: profile_name_stderr.clone(),
                        timestamp,
                        line: clean_line.clone(),
                        source: "stderr".to_string(),
                    };

                    // Store in buffer
                    {
                        let mut buffer = CONSOLE_BUFFER.lock().await;
                        buffer.push_line(&profile_name_stderr, console_line.clone());
                    }

                    // Write to log file (using same log file, different source marker)
                    {
                        let mut log = log_file_stderr.lock().await;
                        let log_entry = format!(
                            "[{}] [stderr] {}\n",
                            timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
                            clean_line
                        );
                        let _ = log.write_all(log_entry.as_bytes());
                    }

                    // Emit event
                    let _ = app_stderr.emit("console-output", &console_line);
                }
                Err(e) => {
                    error!("Error reading stderr: {}", e);
                    break;
                }
            }
        }
    });

    // Emit server-started event
    let _ = app.emit("server-started", &handle);

    // Send system notification
    notify_server_started(&app, &profile_name);

    Ok(handle)
}

/// Stops the ARK server for the given profile.
///
/// First attempts a graceful shutdown via DoExit RCON command. If that fails
/// or times out, kills the process with taskkill.
#[tauri::command]
pub async fn stop_server(profile_name: String, app: AppHandle) -> Result<(), String> {
    info!("stop_server called for profile: {}", profile_name);

    // Get the server handle
    let handle = {
        let state = SERVER_STATE.lock().await;
        state.get_handle(&profile_name).cloned()
    };

    let handle = handle.ok_or_else(|| {
        error!("No server running for profile '{}'", profile_name);
        format!("No server running for profile '{}'", profile_name)
    })?;

    // Set status to Stopping
    {
        let mut state = SERVER_STATE.lock().await;
        state.set_status(&profile_name, ServerStatus::Stopping);
    }

    info!(
        "Stopping server for profile '{}' (PID: {})",
        profile_name, handle.pid
    );

    // Attempt graceful shutdown via taskkill /PID {pid} (SIGTERM equivalent)
    // ARK doesn't have a clean RCON DoExit via command line, so we use taskkill
    let graceful_result = std::process::Command::new("taskkill")
        .args(["/PID", &handle.pid.to_string()])
        .output();

    match graceful_result {
        Ok(output) => {
            if output.status.success() {
                info!(
                    "Graceful stop sent to server '{}' (PID: {})",
                    profile_name, handle.pid
                );
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                debug!("Graceful stop failed (may already be stopped): {}", stderr);
            }
        }
        Err(e) => {
            debug!("taskkill failed: {}", e);
        }
    }

    // Wait briefly then check if process is gone
    let wait_result = tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        tokio::process::Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "Get-Process -Id {} -ErrorAction SilentlyContinue",
                    handle.pid
                ),
            ])
            .output(),
    )
    .await;

    let still_running = match wait_result {
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.contains(&handle.pid.to_string())
        }
        _ => true, // Assume still running if we couldn't check
    };

    // If still running, force kill
    if still_running {
        warn!(
            "Server '{}' did not stop gracefully, force killing (PID: {})",
            profile_name, handle.pid
        );
        let _ = std::process::Command::new("taskkill")
            .args(["/PID", &handle.pid.to_string(), "/F"])
            .output();
    }

    // Remove from state
    {
        let mut state = SERVER_STATE.lock().await;
        state.remove(&profile_name);
    }

    // Clear the console buffer for this profile
    {
        let mut buffer = CONSOLE_BUFFER.lock().await;
        buffer.remove(&profile_name);
    }

    info!("Server stopped: profile='{}'", profile_name);

    // Emit server-stopped event
    let _ = app.emit("server-stopped", &profile_name);

    // Send system notification
    notify_server_stopped(&app, &profile_name);

    Ok(())
}

/// Gets the current status of the server for the given profile.
///
/// Performs actual status detection by checking:
/// - If handle exists but process dead → `Crashed`
/// - If handle exists and process alive → `Running`
/// - If port check succeeds but no handle → `Running` (recovered case)
/// - If port fails and no handle → `Stopped`
#[tauri::command]
pub async fn get_server_status(profile_name: String) -> ServerStatus {
    let (status, _changed) = {
        let state = SERVER_STATE.lock().await;
        // Get port from handle if exists, otherwise use 0 for port check (will fail)
        let port = state.get_handle(&profile_name).map(|h| h.port).unwrap_or(0);
        state.detect_status(&profile_name, port)
    };
    status
}

/// Gets the console buffer (backscroll) for a profile.
/// Returns up to 1000 lines of buffered console output.
#[tauri::command]
pub async fn get_console_buffer(profile_name: String) -> Vec<ConsoleLine> {
    let buffer = CONSOLE_BUFFER.lock().await;
    buffer.get_lines(&profile_name)
}

//! SteamCMD process runner with progress callbacks.

use crate::services::steam_errors::{parse_exit_status, SteamCmdError};
use crate::services::steam_progress::parse_progress_line;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Progress callback type.
pub type ProgressCallback = Box<dyn Fn(SteamProgressOutput) + Send>;

/// Progress output for callbacks.
#[derive(Debug, Clone)]
pub struct SteamProgressOutput {
    /// Download percentage (0-100).
    pub percentage: Option<f64>,
    /// Download speed as string.
    pub speed: Option<String>,
    /// ETA in seconds.
    pub eta_seconds: Option<u64>,
    /// Full output line.
    pub line: String,
}

/// SteamCMD runner configuration.
#[derive(Debug, Clone)]
pub struct RunnerConfig {
    /// Path to steamcmd binary.
    pub steamcmd_path: PathBuf,
    /// Installation directory for ARK server.
    pub install_dir: Option<PathBuf>,
    /// Timeout for steamcmd operations (in seconds).
    pub timeout_secs: Option<u64>,
    /// Run with +login anonymous (default true).
    pub anonymous_login: bool,
    /// Extra args to pass to steamcmd.
    pub extra_args: Vec<String>,
}

impl RunnerConfig {
    /// Create a new runner config with the specified steamcmd path.
    pub fn new(steamcmd_path: PathBuf) -> Self {
        Self {
            steamcmd_path,
            install_dir: None,
            timeout_secs: Some(3600), // 1 hour default
            anonymous_login: true,
            extra_args: vec![],
        }
    }

    /// Set the install directory.
    pub fn with_install_dir(mut self, dir: PathBuf) -> Self {
        self.install_dir = Some(dir);
        self
    }

    /// Disable anonymous login (not implemented, kept for API).
    #[allow(dead_code)]
    pub fn with_login(mut self, _username: &str, _password: &str) -> Self {
        self.anonymous_login = true; // Currently only anonymous supported
        self
    }
}

/// Run steamcmd with a script and return the output.
pub fn run_steamcmd_script(
    config: &RunnerConfig,
    script_lines: &[&str],
    progress_callback: Option<ProgressCallback>,
) -> Result<String, SteamCmdError> {
    let mut cmd = Command::new(&config.steamcmd_path);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.stdin(Stdio::piped());

    // Build the script
    let script = build_script(script_lines, config);
    cmd.arg(script);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = cmd.spawn().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            SteamCmdError::NotFound
        } else {
            SteamCmdError::IoError(e.to_string())
        }
    })?;

    // Collect output - use Arc<Mutex<Vec<String>>> for shared output between threads
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let output_lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    // Read stdout in a separate thread if callback is provided
    let rx = if let Some(callback) = progress_callback {
        let output_clone: Arc<Mutex<Vec<String>>> = Arc::clone(&output_lines);
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            if let Some(stdout) = stdout {
                let reader = BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    if let Some(progress) = parse_progress_line(&line) {
                        callback(SteamProgressOutput {
                            percentage: progress.percentage,
                            speed: progress.speed_bps.map(format_speed),
                            eta_seconds: progress.eta_seconds,
                            line: progress.raw_line,
                        });
                    }
                    output_clone.lock().unwrap().push(line);
                }
            }
            let _ = tx.send(());
        });
        Some(rx)
    } else {
        None
    };

    // Also read stderr and add to output
    if let Some(stderr) = stderr {
        let reader = BufReader::new(stderr);
        for line in reader.lines().map_while(Result::ok) {
            output_lines
                .lock()
                .unwrap()
                .push(format!("[stderr] {}", line));
        }
    }

    // Wait for completion with timeout
    let exit_status = if let Some(timeout) = config.timeout_secs {
        match wait_for_child_with_timeout(&mut child, Duration::from_secs(timeout)) {
            Ok(status) => status,
            Err(_) => {
                child.kill().ok();
                return Err(SteamCmdError::Timeout(timeout));
            }
        }
    } else {
        child
            .wait()
            .map_err(|e| SteamCmdError::IoError(e.to_string()))?
    };

    // Wait for stdout thread to finish
    if let Some(rx) = rx {
        let _ = rx.recv_timeout(Duration::from_secs(5));
    }

    // Check exit status
    if let Some(error) = parse_exit_status(exit_status) {
        return Err(error);
    }

    let result = output_lines.lock().unwrap().join("\n");
    Ok(result)
}

/// Build a steamcmd script string from lines.
fn build_script(lines: &[&str], config: &RunnerConfig) -> String {
    let mut script = String::new();

    if config.anonymous_login {
        script.push_str("login anonymous\n");
    }

    for line in lines {
        script.push_str(line);
        script.push('\n');
    }

    script.push_str("quit\n");

    script
}

/// Wait for child process with timeout.
#[allow(clippy::unnecessary_wraps)]
fn wait_for_child_with_timeout(
    child: &mut Child,
    timeout: Duration,
) -> Result<std::process::ExitStatus, ()> {
    use std::thread;
    use std::time::Instant;

    let start = Instant::now();
    let timeout_duration = timeout;

    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) => {
                if start.elapsed() >= timeout_duration {
                    return Err(());
                }
                thread::sleep(Duration::from_millis(100));
            }
            Err(_) => return Err(()),
        }
    }
}

/// Format speed in human-readable format.
fn format_speed(bytes_per_sec: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes_per_sec >= GB {
        format!("{:.1} GB/s", bytes_per_sec as f64 / GB as f64)
    } else if bytes_per_sec >= MB {
        format!("{:.1} MB/s", bytes_per_sec as f64 / MB as f64)
    } else if bytes_per_sec >= KB {
        format!("{:.1} KB/s", bytes_per_sec as f64 / KB as f64)
    } else {
        format!("{} B/s", bytes_per_sec)
    }
}

/// Verify server installation by running +verify command.
#[allow(dead_code)]
pub fn verify_install(steamcmd_path: &Path, install_dir: &Path) -> Result<bool, SteamCmdError> {
    let config =
        RunnerConfig::new(steamcmd_path.to_path_buf()).with_install_dir(install_dir.to_path_buf());

    let script = format!(
        "force_install_dir \"{}\"\napp_update 376030 validate",
        install_dir.to_string_lossy()
    );

    let output = run_steamcmd_script(&config, &[&script], None)?;
    Ok(output.contains("Success") || output.contains("Verification complete"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_script() {
        let config = RunnerConfig::new(PathBuf::from("/usr/bin/steamcmd"));
        let lines = ["app_update 376030"];
        let script = build_script(&lines, &config);

        assert!(script.contains("login anonymous"));
        assert!(script.contains("app_update 376030"));
        assert!(script.contains("quit"));
    }

    #[test]
    fn test_format_speed() {
        assert_eq!(format_speed(500), "500 B/s");
        assert_eq!(format_speed(1024), "1.0 KB/s");
        assert_eq!(format_speed(1024 * 1024), "1.0 MB/s");
    }
}

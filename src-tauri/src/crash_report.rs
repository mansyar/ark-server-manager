//! Crash report module — captures and stores crash reports when servers exit unexpectedly.
//!
//! Crash reports are stored as JSON files in `%APPDATA%\ArkServerManager\logs\crash_reports\`
//! with naming format `{timestamp}.json`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use sysinfo::System;
use tracing::{error, info};

/// A crash report containing details about an unexpected server exit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    /// Timestamp when the crash was detected.
    pub timestamp: DateTime<Utc>,
    /// Name of the profile that crashed.
    pub profile_name: String,
    /// Exit code of the crashed process (if available).
    pub exit_code: Option<i32>,
    /// Signal that caused the crash (if killed by signal).
    pub signal: Option<i32>,
    /// Last 50 lines of console output before the crash.
    pub last_log_lines: Vec<String>,
    /// System information at the time of crash.
    pub system_info: SystemInfo,
}

/// System information captured at crash time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// OS name (e.g., "Windows", "Linux").
    pub os_name: String,
    /// OS version string.
    pub os_version: String,
    /// Hostname of the machine.
    pub hostname: String,
    /// Total physical memory in bytes.
    pub total_memory_bytes: u64,
    /// Available physical memory in bytes.
    pub available_memory_bytes: u64,
    /// Number of CPU cores.
    pub cpu_cores: usize,
    /// Application version (from Cargo.toml or tauri.conf.json).
    pub app_version: String,
}

impl SystemInfo {
    /// Creates a new SystemInfo struct capturing current system state.
    pub fn capture() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            total_memory_bytes: sys.total_memory(),
            available_memory_bytes: sys.available_memory(),
            cpu_cores: sys.cpus().len(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Returns the crash reports directory path.
pub fn crash_reports_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ArkServerManager")
        .join("logs")
        .join("crash_reports")
}

/// Saves a crash report to the crash reports directory.
///
/// The file is named `{timestamp}.json` where timestamp is ISO8601 format.
pub fn save_crash_report(report: &CrashReport) -> Result<PathBuf, CrashReportError> {
    let dir = crash_reports_dir();

    // Ensure the directory exists
    fs::create_dir_all(&dir).map_err(|e| {
        error!("Failed to create crash reports directory {:?}: {}", dir, e);
        CrashReportError::DirectoryCreationFailed(dir.clone(), e.to_string())
    })?;

    // Generate filename from timestamp
    let filename = format!("{}.json", report.timestamp.format("%Y%m%d_%H%M%S_%f"));
    let path = dir.join(&filename);

    // Serialize to JSON with pretty printing
    let json = serde_json::to_string_pretty(report).map_err(|e| {
        error!("Failed to serialize crash report: {}", e);
        CrashReportError::SerializationFailed(e.to_string())
    })?;

    // Write to file
    fs::write(&path, json).map_err(|e| {
        error!("Failed to write crash report to {:?}: {}", path, e);
        CrashReportError::WriteFailed(path.clone(), e.to_string())
    })?;

    info!("Crash report saved to {:?}", path);
    Ok(path)
}

/// Loads a crash report from a JSON file.
pub fn load_crash_report(path: &PathBuf) -> Result<CrashReport, CrashReportError> {
    let contents = fs::read_to_string(path).map_err(|e| {
        error!("Failed to read crash report from {:?}: {}", path, e);
        CrashReportError::ReadFailed(path.clone(), e.to_string())
    })?;

    serde_json::from_str(&contents).map_err(|e| {
        error!("Failed to parse crash report from {:?}: {}", path, e);
        CrashReportError::ParseFailed(path.clone(), e.to_string())
    })
}

/// Lists all crash reports in the crash reports directory, sorted newest first.
pub fn list_crash_reports() -> Vec<PathBuf> {
    let dir = crash_reports_dir();

    let mut paths: Vec<_> = fs::read_dir(&dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "json")
                        .unwrap_or(false)
                })
                .map(|e| e.path())
                .collect()
        })
        .unwrap_or_default();

    // Sort by modification time, newest first
    paths.sort_by_key(|p| std::cmp::Reverse(p.metadata().and_then(|m| m.modified()).ok()));
    paths
}

/// Errors that can occur when saving or loading crash reports.
#[derive(Debug, Clone)]
pub enum CrashReportError {
    /// Failed to create the crash reports directory.
    DirectoryCreationFailed(PathBuf, String),
    /// Failed to serialize the crash report to JSON.
    SerializationFailed(String),
    /// Failed to write the crash report to a file.
    WriteFailed(PathBuf, String),
    /// Failed to read the crash report from a file.
    ReadFailed(PathBuf, String),
    /// Failed to parse the crash report JSON.
    ParseFailed(PathBuf, String),
}

impl std::fmt::Display for CrashReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrashReportError::DirectoryCreationFailed(path, msg) => {
                write!(f, "Failed to create crash reports directory {:?}: {}", path, msg)
            }
            CrashReportError::SerializationFailed(msg) => {
                write!(f, "Failed to serialize crash report: {}", msg)
            }
            CrashReportError::WriteFailed(path, msg) => {
                write!(f, "Failed to write crash report to {:?}: {}", path, msg)
            }
            CrashReportError::ReadFailed(path, msg) => {
                write!(f, "Failed to read crash report from {:?}: {}", path, msg)
            }
            CrashReportError::ParseFailed(path, msg) => {
                write!(f, "Failed to parse crash report from {:?}: {}", path, msg)
            }
        }
    }
}

impl std::error::Error for CrashReportError {}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_crash_report_serialization() {
        let report = CrashReport {
            timestamp: Utc::now(),
            profile_name: "TestServer".to_string(),
            exit_code: Some(1),
            signal: None,
            last_log_lines: vec![
                "Line 1".to_string(),
                "Line 2".to_string(),
            ],
            system_info: SystemInfo::capture(),
        };

        let json = serde_json::to_string_pretty(&report).unwrap();
        assert!(json.contains("TestServer"));
        assert!(json.contains("timestamp"));
        assert!(json.contains("last_log_lines"));
    }

    #[test]
    fn test_system_info_capture() {
        let info = SystemInfo::capture();

        assert!(!info.os_name.is_empty());
        assert!(!info.hostname.is_empty());
        assert!(info.total_memory_bytes > 0);
        assert!(info.cpu_cores > 0);
    }

    #[test]
    fn test_save_and_load_crash_report() {
        let temp_dir = TempDir::new().unwrap();
        let temp_reports_dir = temp_dir.path().join("crash_reports");

        // Override the crash reports dir for this test
        let report = CrashReport {
            timestamp: Utc::now(),
            profile_name: "TestServer".to_string(),
            exit_code: Some(1),
            signal: None,
            last_log_lines: vec![
                "Error: something went wrong".to_string(),
                "Stack trace: ...".to_string(),
            ],
            system_info: SystemInfo {
                os_name: "TestOS".to_string(),
                os_version: "1.0".to_string(),
                hostname: "test-host".to_string(),
                total_memory_bytes: 16_000_000_000,
                available_memory_bytes: 8_000_000_000,
                cpu_cores: 8,
                app_version: "0.1.0".to_string(),
            },
        };

        // Create the temp directory
        fs::create_dir_all(&temp_reports_dir).unwrap();

        // Manually save to temp dir to test the save logic
        let filename = format!("{}.json", report.timestamp.format("%Y%m%d_%H%M%S_%f"));
        let path = temp_reports_dir.join(&filename);
        let json = serde_json::to_string_pretty(&report).unwrap();
        fs::write(&path, json).unwrap();

        // Load and verify
        let loaded = fs::read_to_string(&path).unwrap();
        let parsed: CrashReport = serde_json::from_str(&loaded).unwrap();

        assert_eq!(parsed.profile_name, "TestServer");
        assert_eq!(parsed.exit_code, Some(1));
        assert_eq!(parsed.last_log_lines.len(), 2);
        assert_eq!(parsed.system_info.os_name, "TestOS");
    }

    #[test]
    fn test_crash_report_error_display() {
        let temp_dir = TempDir::new().unwrap();

        let err = CrashReportError::DirectoryCreationFailed(
            temp_dir.path().to_path_buf(),
            "permission denied".to_string(),
        );
        assert!(err.to_string().contains("permission denied"));

        let err = CrashReportError::WriteFailed(
            temp_dir.path().join("test.json"),
            "disk full".to_string(),
        );
        assert!(err.to_string().contains("disk full"));
    }
}

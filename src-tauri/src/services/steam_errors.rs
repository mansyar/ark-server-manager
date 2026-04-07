//! SteamCMD error types with user-friendly messages.

use std::fmt;
use std::process::ExitStatus;

/// SteamCMD-specific error variants.
#[derive(Debug, Clone)]
pub enum SteamCmdError {
    /// SteamCMD binary not found.
    NotFound,
    /// Installation failed.
    InstallFailed(String),
    /// Update check failed.
    UpdateCheckFailed(String),
    /// Server is currently running, cannot update.
    ServerRunning,
    /// Validation failed after install/update.
    ValidationFailed(String),
    /// Download failed after max retries.
    DownloadFailed(String),
    /// Path traversal attempt detected.
    PathTraversal,
    /// Timeout waiting for steamcmd.
    Timeout(u64),
    /// SteamCMD exited with an error.
    ExitCode(i32),
    /// IO error.
    IoError(String),
    /// Network error during download.
    NetworkError(String),
}

impl fmt::Display for SteamCmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SteamCmdError::NotFound => {
                write!(f, "SteamCMD not found. Please install SteamCMD first.")
            }
            SteamCmdError::InstallFailed(msg) => {
                write!(f, "SteamCMD installation failed: {}. Check your internet connection and try again.", msg)
            }
            SteamCmdError::UpdateCheckFailed(msg) => {
                write!(
                    f,
                    "Failed to check for updates: {}. SteamCMD may be corrupted.",
                    msg
                )
            }
            SteamCmdError::ServerRunning => {
                write!(
                    f,
                    "Cannot update server while it is running. Stop the server first."
                )
            }
            SteamCmdError::ValidationFailed(msg) => {
                write!(
                    f,
                    "File validation failed: {}. Some files may be corrupted.",
                    msg
                )
            }
            SteamCmdError::DownloadFailed(msg) => {
                write!(f, "Download failed after multiple attempts: {}", msg)
            }
            SteamCmdError::PathTraversal => {
                write!(f, "Invalid install path. Path traversal is not allowed.")
            }
            SteamCmdError::Timeout(secs) => {
                write!(f, "SteamCMD timed out after {} seconds.", secs)
            }
            SteamCmdError::ExitCode(code) => {
                write!(f, "SteamCMD exited with error code: {}", code)
            }
            SteamCmdError::IoError(msg) => {
                write!(f, "IO error: {}", msg)
            }
            SteamCmdError::NetworkError(msg) => {
                write!(f, "Network error: {}. Check your internet connection.", msg)
            }
        }
    }
}

impl std::error::Error for SteamCmdError {}

impl From<std::io::Error> for SteamCmdError {
    fn from(err: std::io::Error) -> Self {
        SteamCmdError::IoError(err.to_string())
    }
}

impl From<reqwest::Error> for SteamCmdError {
    fn from(err: reqwest::Error) -> Self {
        SteamCmdError::NetworkError(err.to_string())
    }
}

impl From<zip::result::ZipError> for SteamCmdError {
    fn from(err: zip::result::ZipError) -> Self {
        SteamCmdError::InstallFailed(format!("Zip error: {}", err))
    }
}

/// Parse steamcmd exit status into error.
pub fn parse_exit_status(status: ExitStatus) -> Option<SteamCmdError> {
    if status.success() {
        None
    } else if status.code().is_some() {
        Some(SteamCmdError::ExitCode(status.code().unwrap()))
    } else {
        // Signal death
        Some(SteamCmdError::ExitCode(-1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SteamCmdError::NotFound;
        assert!(err.to_string().contains("SteamCMD not found"));
    }

    #[test]
    fn test_server_running_message() {
        let err = SteamCmdError::ServerRunning;
        assert!(err.to_string().contains("running"));
    }

    #[test]
    fn test_path_traversal_message() {
        let err = SteamCmdError::PathTraversal;
        assert!(err.to_string().contains("Path traversal"));
    }
}

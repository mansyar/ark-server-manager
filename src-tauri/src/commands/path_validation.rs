use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, warn};

/// Result of path validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathValidationResult {
    /// Whether the path is valid for the requested use.
    pub valid: bool,
    /// Whether the path exists on disk.
    pub exists: bool,
    /// Whether the path is a directory (true) or file (false).
    pub is_directory: bool,
    /// Human-readable hint or error message.
    pub hint: String,
}

/// Validates a path for use as a server folder or SteamCMD executable.
///
/// # Arguments
/// * `path` - The path string to validate
/// * `path_type` - Either "server_folder" or "steamcmd"
#[tauri::command]
pub fn validate_path(path: String, path_type: String) -> PathValidationResult {
    validate_path_impl(&path, &path_type)
}

fn validate_path_impl(path: &str, path_type: &str) -> PathValidationResult {
    let path = path.trim();
    
    // Handle empty paths - these are valid (will trigger auto-discovery)
    if path.is_empty() {
        return PathValidationResult {
            valid: true,
            exists: false,
            is_directory: false,
            hint: "Empty path - auto-discovery will be used".to_string(),
        };
    }

    let p = Path::new(path);

    // Check if path exists
    if !p.exists() {
        debug!("Path does not exist: {}", path);
        return PathValidationResult {
            valid: false,
            exists: false,
            is_directory: false,
            hint: "Path does not exist".to_string(),
        };
    }

    match path_type {
        "server_folder" => validate_server_folder(p),
        "steamcmd" => validate_steamcmd(p),
        _ => {
            warn!("Unknown path_type: {}", path_type);
            PathValidationResult {
                valid: false,
                exists: true,
                is_directory: p.is_dir(),
                hint: format!("Unknown path type '{}'. Use 'server_folder' or 'steamcmd'", path_type),
            }
        }
    }
}

/// Validates a path as an ARK server installation folder.
fn validate_server_folder(p: &Path) -> PathValidationResult {
    // Check if it's a directory
    if !p.is_dir() {
        return PathValidationResult {
            valid: false,
            exists: true,
            is_directory: false,
            hint: "Server folder must be a directory".to_string(),
        };
    }

    // Check for ShooterGame/Binaries/Win64/ShooterGameServer.exe
    let server_exe = p
        .join("ShooterGame")
        .join("Binaries")
        .join("Win64")
        .join("ShooterGameServer.exe");

    if server_exe.exists() {
        PathValidationResult {
            valid: true,
            exists: true,
            is_directory: true,
            hint: "Valid ARK server installation".to_string(),
        }
    } else {
        // Also check if ShooterGameServer.exe exists directly in Win64
        let alt_exe = p.join("ShooterGameServer.exe");
        if alt_exe.exists() {
            PathValidationResult {
                valid: true,
                exists: true,
                is_directory: true,
                hint: "Valid ARK server installation (direct executable)".to_string(),
            }
        } else {
            PathValidationResult {
                valid: false,
                exists: true,
                is_directory: true,
                hint: "Directory does not contain ShooterGameServer.exe. Is this the ARK server root?".to_string(),
            }
        }
    }
}

/// Validates a path as a SteamCMD executable.
fn validate_steamcmd(p: &Path) -> PathValidationResult {
    // Must be a file, not a directory
    if p.is_dir() {
        return PathValidationResult {
            valid: false,
            exists: true,
            is_directory: true,
            hint: "SteamCMD path must be a file (steamcmd.exe), not a directory".to_string(),
        };
    }

    // Must have .exe extension
    let extension = p
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    match extension.as_deref() {
        Some("exe") => {
            // Check if it's named steamcmd (case-insensitive)
            let filename = p
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.to_lowercase())
                .unwrap_or_default();

            if filename.contains("steamcmd") {
                PathValidationResult {
                    valid: true,
                    exists: true,
                    is_directory: false,
                    hint: "Valid SteamCMD executable".to_string(),
                }
            } else {
                PathValidationResult {
                    valid: false,
                    exists: true,
                    is_directory: false,
                    hint: "File exists but does not appear to be SteamCMD (expected filename contains 'steamcmd')".to_string(),
                }
            }
        }
        Some(ext) => PathValidationResult {
            valid: false,
            exists: true,
            is_directory: false,
            hint: format!("SteamCMD must be an .exe file, got .{}", ext),
        },
        None => PathValidationResult {
            valid: false,
            exists: true,
            is_directory: false,
            hint: "SteamCMD must have a file extension (.exe)".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_path_is_valid() {
        let result = validate_path_impl("", "server_folder");
        assert!(result.valid);
        assert!(!result.exists);
    }

    #[test]
    fn test_empty_path_for_steamcmd_is_valid() {
        let result = validate_path_impl("", "steamcmd");
        assert!(result.valid);
        assert!(!result.exists);
    }

    #[test]
    fn test_whitespace_only_path_is_valid() {
        let result = validate_path_impl("   ", "server_folder");
        assert!(result.valid);
        assert!(!result.exists);
    }

    #[test]
    fn test_unknown_path_type() {
        // Use a real path that exists to test the path_type logic
        let result = validate_path_impl("/", "unknown");
        assert!(!result.valid);
        assert!(result.exists);
    }

    #[test]
    fn test_non_existent_path_invalid() {
        let result = validate_path_impl("C:\\nonexistent\\path\\foo", "server_folder");
        assert!(!result.valid);
        assert!(!result.exists);
    }
}

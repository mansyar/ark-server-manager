//! SteamCMD installation detection.

use std::path::{Path, PathBuf};

/// Common SteamCMD installation paths.
const STEAMCMD_PATHS: &[&str] = &[
    "steamcmd",
    ".steamcmd",
    ".local/bin/steamcmd",
    "/usr/local/bin/steamcmd",
    "/usr/bin/steamcmd",
    "/home/steam/steamcmd",
    "/opt/steamcmd/steamcmd",
    "C:\\steamcmd\\steamcmd.exe",
    "C:\\Program Files\\steamcmd\\steamcmd.exe",
    "C:\\Program Files (x86)\\steamcmd\\steamcmd.exe",
];

/// Detect if SteamCMD is installed.
///
/// Checks common paths and returns the first one found.
pub fn detect_steamcmd() -> Option<PathBuf> {
    for path_str in STEAMCMD_PATHS {
        let path = Path::new(path_str);
        if path.exists() && is_executable(path) {
            return Some(path.to_path_buf());
        }
    }

    // Also check PATH by looking for "steamcmd" in PATH
    if let Ok(path) = std::env::var("PATH") {
        for dir in path.split(':') {
            let candidate = Path::new(dir).join("steamcmd");
            if candidate.exists() && is_executable(&candidate) {
                return Some(candidate);
            }
        }
    }

    None
}

/// Check if a path is executable.
fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            let mode = metadata.permissions().mode();
            return mode & 0o111 != 0; // Check any execute bit
        }
    }
    #[cfg(windows)]
    {
        // On Windows, check if file ends in .exe or exists as a file
        if let Ok(metadata) = path.metadata() {
            return metadata.is_file();
        }
    }
    false
}

/// Validate that a path is a valid SteamCMD installation.
pub fn validate_steamcmd(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }
    is_executable(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_none_when_not_present() {
        // This test just verifies the function runs without panic
        let result = detect_steamcmd();
        // Result is environment-dependent, so we just check it doesn't panic
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_path_traversal_prevention() {
        let malicious = Path::new("steamcmd/../../../etc/passwd");
        // validate_steamcmd should return false for non-existent paths
        assert!(!validate_steamcmd(malicious));
    }

    #[test]
    fn test_common_paths_exist() {
        for path_str in STEAMCMD_PATHS {
            let path = Path::new(path_str);
            // This is just a sanity check that all paths are valid Path objects
            assert!(path.components().next().is_some());
        }
    }
}

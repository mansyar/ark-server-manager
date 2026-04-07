//! Server installation discovery and path management.
//!
//! Handles discovery of ARK server binaries at their default Windows paths
//! and provides validation for user-specified custom paths.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Default ARK server installation paths on Windows.
const DEFAULT_ARK_EXE: &str =
    r#"C:\Program Files (x86)\Steam\steamapps\common\ARK\ShooterGame\Binaries\Win64\ShooterGameServer.exe"#;
const DEFAULT_STEAMCMD: &str =
    r#"C:\Program Files (x86)\Steam\steamapps\common\ARK\Engine\Binaries\ThirdParty\SteamCMD\steamcmd.exe"#;

/// Represents a discovered ARK server installation.
/// Contains paths to the key binaries and the base install directory.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerInstall {
    /// Path to the SteamCMD binary.
    pub steamcmd_path: PathBuf,
    /// Path to the ShooterGameServer.exe binary.
    pub ark_exe_path: PathBuf,
    /// Base installation directory (parent of the above paths).
    pub install_dir: PathBuf,
}

/// Errors that can occur during server installation discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryError {
    /// The ARK server installation was not found at the default path.
    /// Contains user-facing guidance on how to resolve the issue.
    InstallNotFound {
        guidance: String,
        missing_executable: String,
    },
}

impl std::fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiscoveryError::InstallNotFound {
                guidance,
                missing_executable,
            } => {
                write!(
                    f,
                    "ARK server not found. Missing: {}. {}",
                    missing_executable, guidance
                )
            }
        }
    }
}

impl std::error::Error for DiscoveryError {}

/// Result of validating a server installation for a specific profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the installation is valid.
    pub is_valid: bool,
    /// Human-readable message describing the validation result.
    pub message: String,
    /// Resolved path to the ARK executable.
    pub ark_exe_path: PathBuf,
}

/// Returns the default paths for ARK server binaries.
fn default_paths() -> (PathBuf, PathBuf) {
    let ark_exe = PathBuf::from(DEFAULT_ARK_EXE);
    let steamcmd = PathBuf::from(DEFAULT_STEAMCMD);
    (ark_exe, steamcmd)
}

/// Checks if a path exists and is a file.
fn path_is_valid(p: &Path) -> bool {
    p.exists() && p.is_file()
}

/// Discovers the ARK server installation at default paths.
///
/// Returns `ServerInstall` if both the ARK server exe and SteamCMD are found.
/// Returns `DiscoveryError::InstallNotFound` with guidance if either is missing.
///
/// # Examples
///
/// ```
/// use ark_server_manager::services::server_discovery::discover_server_install;
///
/// let result = discover_server_install();
/// match result {
///     Ok(install) => println!("Found ARK at {:?}", install.ark_exe_path),
///     Err(e) => eprintln!("Discovery failed: {}", e),
/// }
/// ```
pub fn discover_server_install() -> Result<ServerInstall, DiscoveryError> {
    let (ark_exe_path, steamcmd_path) = default_paths();

    info!(
        "Discovering ARK server installation at default paths: ark={}, steamcmd={}",
        ark_exe_path.display(),
        steamcmd_path.display()
    );

    let ark_exists = path_is_valid(&ark_exe_path);
    let steamcmd_exists = path_is_valid(&steamcmd_path);

    debug!(
        "ARK exe exists: {}, SteamCMD exists: {}",
        ark_exists, steamcmd_exists
    );

    // Determine which executable is missing for better guidance
    if !ark_exists && !steamcmd_exists {
        warn!(
            "Neither ARK exe nor SteamCMD found at default locations"
        );
        return Err(DiscoveryError::InstallNotFound {
            guidance: "ARK server files not found. Please ensure SteamCMD has downloaded the ARK server files, or use a custom server_install_path in your profile.".to_string(),
            missing_executable: "ShooterGameServer.exe AND steamcmd.exe".to_string(),
        });
    }

    if !ark_exists {
        warn!("ARK exe not found at default location");
        return Err(DiscoveryError::InstallNotFound {
            guidance: "ShooterGameServer.exe not found. Please verify ARK server files are installed, or specify a custom server_install_path in your profile.".to_string(),
            missing_executable: "ShooterGameServer.exe".to_string(),
        });
    }

    if !steamcmd_exists {
        warn!("SteamCMD not found at default location");
        return Err(DiscoveryError::InstallNotFound {
            guidance: "steamcmd.exe not found. Please verify SteamCMD is installed in the ARK directory, or specify a custom server_install_path in your profile.".to_string(),
            missing_executable: "steamcmd.exe".to_string(),
        });
    }

    // Both found - determine install_dir from the ARK exe path
    // The structure is: .../ARK/ShooterGame/Binaries/Win64/ShooterGameServer.exe
    // So install_dir should be the ARK root: .../ARK/
    let install_dir = ark_exe_path
        .ancestors()
        .find(|p| p.file_name().is_some_and(|n| n == "ARK"))
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            // Fallback: go up 4 levels from Win64\ShooterGameServer.exe
            ark_exe_path.ancestors().nth(3).map(PathBuf::from).unwrap_or_default()
        });

    let install = ServerInstall {
        steamcmd_path,
        ark_exe_path,
        install_dir,
    };

    info!(
        "Discovered ARK server installation at {:?}",
        install.ark_exe_path
    );

    Ok(install)
}

/// Resolves the ARK executable path for a profile.
///
/// Uses the profile's `server_install_path` if set and valid,
/// otherwise falls back to the default discovery.
pub fn resolve_ark_exe(profile: &crate::models::Profile) -> Result<PathBuf, DiscoveryError> {
    if let Some(ref custom_path) = profile.server_install_path {
        debug!(
            "Using custom server_install_path from profile: {}",
            profile.name
        );

        // Custom path specified - validate it points to ShooterGameServer.exe
        if !path_is_valid(custom_path) {
            warn!(
                "Custom server_install_path does not exist or is not a file: {:?}",
                custom_path
            );
            return Err(DiscoveryError::InstallNotFound {
                guidance: format!(
                    "Custom path {:?} does not exist or is not a file. Please verify the path or remove server_install_path from profile '{}' to use default location.",
                    custom_path,
                    profile.name
                ),
                missing_executable: custom_path.display().to_string(),
            });
        }

        // Check if it's ShooterGameServer.exe or a parent directory
        let exe_path = if custom_path.file_name().is_some_and(|n| n == "ShooterGameServer.exe") {
            custom_path.clone()
        } else {
            // Assume it's a directory - look for the exe inside
            custom_path
                .join("ShooterGame\\Binaries\\Win64\\ShooterGameServer.exe")
        };

        if path_is_valid(&exe_path) {
            return Ok(exe_path);
        }

        // Try the parent as install_dir pattern
        return Err(DiscoveryError::InstallNotFound {
            guidance: format!(
                "ShooterGameServer.exe not found at expected location within custom path. Please verify server_install_path in profile '{}' points to the ARK installation directory or ShooterGameServer.exe directly.",
                profile.name
            ),
            missing_executable: "ShooterGameServer.exe".to_string(),
        });
    }

    // No custom path - use default discovery
    let install = discover_server_install()?;
    Ok(install.ark_exe_path)
}

/// Validates the server installation for a given profile name.
pub fn validate_install_for_profile(
    profile_name: &str,
    profiles_dir: PathBuf,
) -> ValidationResult {
    let profile_path = profiles_dir.join(format!("{}.json", profile_name));

    if !profile_path.exists() {
        return ValidationResult {
            is_valid: false,
            message: format!("Profile '{}' not found", profile_name),
            ark_exe_path: PathBuf::new(),
        };
    }

    let contents = match std::fs::read_to_string(&profile_path) {
        Ok(c) => c,
        Err(e) => {
            return ValidationResult {
                is_valid: false,
                message: format!("Failed to read profile '{}': {}", profile_name, e),
                ark_exe_path: PathBuf::new(),
            };
        }
    };

    let profile: crate::models::Profile = match serde_json::from_str(&contents) {
        Ok(p) => p,
        Err(e) => {
            return ValidationResult {
                is_valid: false,
                message: format!("Failed to parse profile '{}': {}", profile_name, e),
                ark_exe_path: PathBuf::new(),
            };
        }
    };

    match resolve_ark_exe(&profile) {
        Ok(ark_exe_path) => ValidationResult {
            is_valid: true,
            message: format!("Installation valid for profile '{}'", profile_name),
            ark_exe_path,
        },
        Err(e) => ValidationResult {
            is_valid: false,
            message: e.to_string(),
            ark_exe_path: PathBuf::new(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_paths_exist() {
        let (ark_exe, steamcmd) = default_paths();
        assert_eq!(
            ark_exe.to_str().unwrap(),
            r"C:\Program Files (x86)\Steam\steamapps\common\ARK\ShooterGame\Binaries\Win64\ShooterGameServer.exe"
        );
        assert_eq!(
            steamcmd.to_str().unwrap(),
            r"C:\Program Files (x86)\Steam\steamapps\common\ARK\Engine\Binaries\ThirdParty\SteamCMD\steamcmd.exe"
        );
    }

    #[test]
    fn test_path_is_valid_for_nonexistent() {
        let fake_path = PathBuf::from("C:\\nonexistent\\fake.exe");
        assert!(!path_is_valid(&fake_path));
    }

    #[test]
    fn test_server_install_equality() {
        let install1 = ServerInstall {
            steamcmd_path: PathBuf::from("steamcmd.exe"),
            ark_exe_path: PathBuf::from("ShooterGameServer.exe"),
            install_dir: PathBuf::from("C:\\ARK"),
        };
        let install2 = ServerInstall {
            steamcmd_path: PathBuf::from("steamcmd.exe"),
            ark_exe_path: PathBuf::from("ShooterGameServer.exe"),
            install_dir: PathBuf::from("C:\\ARK"),
        };
        assert_eq!(install1, install2);
    }

    #[test]
    fn test_discovery_error_display() {
        let err = DiscoveryError::InstallNotFound {
            guidance: "Install ARK server".to_string(),
            missing_executable: "ShooterGameServer.exe".to_string(),
        };
        let display = format!("{}", err);
        assert!(display.contains("ShooterGameServer.exe"));
        assert!(display.contains("Install ARK server"));
    }

    #[test]
    fn test_validation_result_serde() {
        let result = ValidationResult {
            is_valid: true,
            message: "All good".to_string(),
            ark_exe_path: PathBuf::from("C:\\ark\\ShooterGameServer.exe"),
        };
        let json = serde_json::to_string(&result).unwrap();
        let parsed: ValidationResult = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_valid);
        assert_eq!(parsed.message, "All good");
    }
}
//! ARK server update manager.

use crate::services::build_id_checker::{check_build_ids, BuildIdInfo};
use crate::services::retry::{retry, RetryConfig};
use crate::services::steam_errors::SteamCmdError;
use crate::services::steamcmd_runner::{run_steamcmd_script, RunnerConfig};
use std::path::Path;
use std::path::PathBuf;

/// Update manager configuration.
#[derive(Debug, Clone)]
pub struct UpdateManagerConfig {
    /// Path to steamcmd binary.
    pub steamcmd_path: PathBuf,
    /// Server installation directory.
    pub install_dir: PathBuf,
    /// Retry configuration for downloads.
    pub retry_config: RetryConfig,
    /// Whether to validate after update.
    pub validate_after_update: bool,
}

impl Default for UpdateManagerConfig {
    fn default() -> Self {
        Self {
            steamcmd_path: PathBuf::from("steamcmd"),
            install_dir: PathBuf::from("."),
            retry_config: RetryConfig::default(),
            validate_after_update: true,
        }
    }
}

impl UpdateManagerConfig {
    /// Create a new config with required paths.
    pub fn new(steamcmd_path: PathBuf, install_dir: PathBuf) -> Self {
        Self {
            steamcmd_path,
            install_dir,
            ..Default::default()
        }
    }

    /// Set retry config.
    #[allow(dead_code)]
    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Enable/disable post-update validation.
    #[allow(dead_code)]
    pub fn with_validate_after_update(mut self, validate: bool) -> Self {
        self.validate_after_update = validate;
        self
    }
}

/// Update result.
#[derive(Debug, Clone, serde::Serialize)]
pub struct UpdateResult {
    /// Whether an update was performed.
    pub updated: bool,
    /// Previous build ID (if known).
    pub previous_build: Option<u64>,
    /// New build ID (if known).
    pub new_build: Option<u64>,
    /// Whether validation passed.
    pub validation_passed: bool,
    /// Output from steamcmd.
    pub output: String,
    /// Build IDs info for update check.
    pub build_ids: Option<BuildIdInfo>,
}

/// Check if the server is currently running.
///
/// This is a best-effort check. Returns false (not running) if we can't determine.
#[allow(dead_code)]
fn is_server_running(_install_dir: &Path) -> bool {
    // TODO: Integrate with server_state to check if server process is alive
    // For now, return false to allow updates
    false
}

/// Perform a server update.
pub fn update_server(config: UpdateManagerConfig) -> Result<UpdateResult, SteamCmdError> {
    // Check if server is running
    if is_server_running(&config.install_dir) {
        return Err(SteamCmdError::ServerRunning);
    }

    // Check current build ID before update
    let before_info = check_build_ids(&config.steamcmd_path, &config.install_dir)?;
    let previous_build = before_info.current_build;

    // Perform update with retry
    let output = retry(config.retry_config.clone(), || {
        let config = config.clone();
        perform_update(&config)
    })
    .map_err(|_| SteamCmdError::DownloadFailed("Max retries exceeded".to_string()))?;

    // Validate if enabled
    let validation_passed = if config.validate_after_update {
        validate_server_install(&config.steamcmd_path, &config.install_dir)?
    } else {
        true
    };

    // Check build ID after update
    let after_info = check_build_ids(&config.steamcmd_path, &config.install_dir)?;

    let updated = previous_build != after_info.current_build;

    Ok(UpdateResult {
        updated,
        previous_build,
        new_build: after_info.current_build,
        validation_passed,
        output,
        build_ids: Some(before_info),
    })
}

/// Perform the actual update operation.
fn perform_update(config: &UpdateManagerConfig) -> Result<String, SteamCmdError> {
    let runner_config = RunnerConfig::new(config.steamcmd_path.clone())
        .with_install_dir(config.install_dir.clone());

    let script = format!(
        "force_install_dir \"{}\"\napp_update 376030",
        config.install_dir.to_string_lossy()
    );

    run_steamcmd_script(&runner_config, &[&script], None)
}

/// Validate server installation after update.
fn validate_server_install(
    steamcmd_path: &Path,
    install_dir: &Path,
) -> Result<bool, SteamCmdError> {
    let runner_config =
        RunnerConfig::new(steamcmd_path.to_path_buf()).with_install_dir(install_dir.to_path_buf());

    let script = format!(
        "force_install_dir \"{}\"\napp_update 376030 validate",
        install_dir.to_string_lossy()
    );

    let output = run_steamcmd_script(&runner_config, &[&script], None)?;

    Ok(output.contains("Success") || output.contains("Verification complete"))
}

/// Check for available updates without applying them.
#[allow(dead_code)]
pub fn check_for_updates(
    steamcmd_path: &Path,
    install_dir: &Path,
) -> Result<BuildIdInfo, SteamCmdError> {
    check_build_ids(steamcmd_path, install_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = UpdateManagerConfig::default();
        assert_eq!(config.steamcmd_path, PathBuf::from("steamcmd"));
        assert!(config.validate_after_update);
    }

    #[test]
    fn test_with_retry_config() {
        let config = UpdateManagerConfig::default()
            .with_retry_config(RetryConfig::default().with_max_attempts(5));
        assert_eq!(config.retry_config.max_attempts, 5);
    }

    #[test]
    fn test_update_result_debug() {
        let result = UpdateResult {
            updated: true,
            previous_build: Some(1000),
            new_build: Some(1001),
            validation_passed: true,
            output: "Success".to_string(),
            build_ids: None,
        };
        let debug = format!("{:?}", result);
        assert!(debug.contains("updated: true"));
    }
}

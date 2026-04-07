//! Build ID checking for ARK server updates.

use crate::services::steamcmd_runner::{run_steamcmd_script, RunnerConfig};
use regex::Regex;
use std::path::Path;

/// Build ID information.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuildIdInfo {
    /// Current installed build ID.
    pub current_build: Option<u64>,
    /// Available build ID from Steam.
    pub available_build: Option<u64>,
    /// Whether an update is available.
    pub update_available: bool,
}

impl BuildIdInfo {
    /// Returns true if an update is needed.
    pub fn needs_update(&self) -> bool {
        self.update_available
    }
}

/// Check build IDs for an ARK server installation.
pub fn check_build_ids(
    steamcmd_path: &Path,
    install_dir: &Path,
) -> Result<BuildIdInfo, crate::services::steam_errors::SteamCmdError> {
    let config = RunnerConfig::new(steamcmd_path.to_path_buf())
        .with_install_dir(install_dir.to_path_buf());

    let script = format!(
        "force_install_dir \"{}\"\napp_update 376030 -checksrc",
        install_dir.to_string_lossy()
    );

    let output = run_steamcmd_script(&config, &[&script], None)?;

    // Parse build IDs from output
    // Pattern: "Build ID: 12345678"
    let build_id_re = Regex::new(r"Build ID:\s*(\d+)").ok();

    let mut current_build: Option<u64> = None;
    let mut available_build: Option<u64> = None;

    if let Some(re) = build_id_re {
        for line in output.lines() {
            if let Some(caps) = re.captures(line) {
                let build: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
                if current_build.is_none() {
                    current_build = Some(build);
                }
                available_build = Some(build);
            }
        }
    }

    // If both are set, check if they differ
    let update_available = match (current_build, available_build) {
        (Some(current), Some(available)) => current != available,
        _ => false,
    };

    Ok(BuildIdInfo {
        current_build,
        available_build,
        update_available,
    })
}

/// Get just the current build ID without checking for updates.
pub fn get_current_build_id(
    steamcmd_path: &Path,
    install_dir: &Path,
) -> Result<Option<u64>, crate::services::steam_errors::SteamCmdError> {
    let config = RunnerConfig::new(steamcmd_path.to_path_buf())
        .with_install_dir(install_dir.to_path_buf());

    // Run with -complete_done to get info without triggering download
    let script = format!(
        "force_install_dir \"{}\"\napp_status 376030",
        install_dir.to_string_lossy()
    );

    let output = run_steamcmd_script(&config, &[&script], None)?;

    let build_id_re = Regex::new(r"Build ID:\s*(\d+)").ok();

    if let Some(re) = build_id_re {
        for line in output.lines() {
            if let Some(caps) = re.captures(line) {
                return Ok(caps.get(1).unwrap().as_str().parse().ok());
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_id_info_needs_update() {
        let info = BuildIdInfo {
            current_build: Some(1000),
            available_build: Some(1001),
            update_available: true,
        };
        assert!(info.needs_update());
    }

    #[test]
    fn test_build_id_info_no_update() {
        let info = BuildIdInfo {
            current_build: Some(1000),
            available_build: Some(1000),
            update_available: false,
        };
        assert!(!info.needs_update());
    }
}

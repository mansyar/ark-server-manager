//! Profile-related Tauri commands.

pub mod path_validation;
pub mod server_control;
pub mod steam_install;

use crate::models::{Profile, ProfileMetadata};
use crate::services::backup::{create_backup, BackupResult};
use crate::services::notify_backup_completed;
use crate::services::server_discovery::{
    discover_server_install, validate_install_for_profile, ServerInstall, ValidationResult,
};
use crate::services::server_state::load_profile as load_profile_from_state;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
use tauri::AppHandle;
use tracing::{error, info, warn};

/// Returns the profiles directory path.
fn profiles_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ArkServerManager")
        .join("profiles")
}

/// Ensures the profiles directory exists, creating it if missing.
fn ensure_profiles_dir() -> std::io::Result<()> {
    let dir = profiles_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(())
}

/// Lists all saved profiles.
#[tauri::command]
pub fn list_profiles() -> Result<Vec<ProfileMetadata>, String> {
    let dir = profiles_dir();

    if !dir.exists() {
        info!("Profiles directory does not exist yet; returning empty list");
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(&dir).map_err(|e| {
        error!("Failed to read profiles directory: {}", e);
        e.to_string()
    })?;

    let mut profiles = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        // Only consider .json files
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        // Read file to extract metadata (name + map from JSON)
        let mut file = File::open(&path).map_err(|e| {
            warn!("Could not open profile file {:?}: {}", path, e);
            e.to_string()
        })?;

        let mut json_contents = String::new();
        file.read_to_string(&mut json_contents)
            .map_err(|e| e.to_string())?;

        match serde_json::from_str::<Profile>(&json_contents) {
            Ok(p) => {
                let last_modified = entry
                    .metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::SystemTime::now());
                profiles.push(ProfileMetadata {
                    name: p.name,
                    map: p.map,
                    last_modified,
                });
            }
            Err(e) => {
                // Skip malformed profiles gracefully
                warn!("Skipping malformed profile {:?}: {}", path, e);
            }
        }
    }

    info!("list_profiles returning {} profiles", profiles.len());
    Ok(profiles)
}

/// Loads a single profile by exact name.
#[tauri::command]
pub fn load_profile(name: String) -> Result<Profile, String> {
    let path = profiles_dir().join(format!("{}.json", name));

    if !path.exists() {
        error!("Profile not found: {}", name);
        return Err(format!("Profile '{}' not found", name));
    }

    let contents = fs::read_to_string(&path).map_err(|e| {
        error!("Failed to read profile '{}': {}", name, e);
        e.to_string()
    })?;

    let profile: Profile = serde_json::from_str(&contents).map_err(|e| {
        error!("Failed to parse profile '{}': {}", name, e);
        format!("Invalid profile JSON: {}", e)
    })?;

    info!("Loaded profile: {}", name);
    Ok(profile)
}

/// Saves (creates or overwrites) a profile.
#[tauri::command]
pub fn save_profile(profile: Profile) -> Result<(), String> {
    ensure_profiles_dir().map_err(|e| {
        error!("Failed to create profiles directory: {}", e);
        e.to_string()
    })?;

    let path = profiles_dir().join(format!("{}.json", profile.name));

    let json = serde_json::to_string_pretty(&profile).map_err(|e| {
        error!("Failed to serialize profile '{}': {}", profile.name, e);
        format!("Serialization failed: {}", e)
    })?;

    fs::write(&path, json).map_err(|e| {
        error!("Failed to write profile '{}': {}", profile.name, e);
        e.to_string()
    })?;

    info!("Saved profile '{}' to {:?}", profile.name, path);
    Ok(())
}

/// Deletes a profile by exact name.
#[tauri::command]
pub fn delete_profile(name: String) -> Result<(), String> {
    let path = profiles_dir().join(format!("{}.json", name));

    if !path.exists() {
        error!("delete_profile: profile '{}' not found", name);
        return Err(format!("Profile '{}' not found", name));
    }

    fs::remove_file(&path).map_err(|e| {
        error!("Failed to delete profile '{}': {}", name, e);
        e.to_string()
    })?;

    info!("Deleted profile: {}", name);
    Ok(())
}

/// Discovers the ARK server installation at default paths.
///
/// Returns `ServerInstall` if both the ARK server exe and SteamCMD are found.
/// Returns `DiscoveryError::InstallNotFound` with guidance if either is missing.
#[tauri::command]
pub fn discover_install() -> Result<ServerInstall, String> {
    discover_server_install().map_err(|e| e.to_string())
}

/// Validates the server installation for a given profile name.
/// Checks that the ARK executable exists (at default or custom path).
#[tauri::command]
pub fn validate_install(profile_name: String) -> ValidationResult {
    let dir = profiles_dir();
    validate_install_for_profile(&profile_name, dir)
}

/// Triggers a backup for the given profile.
///
/// Loads the profile, resolves the ARK server install directory, creates a
/// timestamped ZIP backup, enforces the `backup_retention_count`, and returns
/// a `BackupResult` describing the outcome.
#[tauri::command]
pub fn trigger_backup(profile_name: String, app: AppHandle) -> BackupResult {
    info!("trigger_backup called for profile: {}", profile_name);

    // Load the profile to get backup settings and resolve source dir
    let profile = match load_profile_from_state(&profile_name) {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to load profile '{}': {}", profile_name, e);
            return BackupResult {
                zip_path: None,
                message: format!("Failed to load profile '{}': {}", profile_name, e),
                backups_retained: 0,
            };
        }
    };

    // Resolve the ARK server root directory
    let source_dir = match crate::services::server_discovery::resolve_ark_exe(&profile) {
        Ok(exe_path) => {
            // The executable is in ShooterGame/Binaries/Win64, so parent is Win64,
            // parent's parent is the root ARK install directory
            exe_path
                .parent()
                .and_then(|p| p.parent())
                .unwrap_or_else(|| exe_path.parent().unwrap_or(&exe_path))
                .to_path_buf()
        }
        Err(e) => {
            error!(
                "Failed to resolve ARK executable for profile '{}': {}",
                profile_name, e
            );
            return BackupResult {
                zip_path: None,
                message: format!("Failed to resolve ARK install path: {}", e),
                backups_retained: 0,
            };
        }
    };

    if !source_dir.exists() {
        error!("ARK server directory {:?} does not exist", source_dir);
        return BackupResult {
            zip_path: None,
            message: format!("ARK server directory does not exist: {:?}", source_dir),
            backups_retained: 0,
        };
    }

    let result = create_backup(
        &source_dir,
        &profile.steamcmd_install_dir,
        &profile.backup_dir,
        &profile.name,
        &profile.backup_suffix,
        profile.backup_retention_count,
    );

    info!(
        "Backup result for '{}': {} (retained: {})",
        profile_name, result.message, result.backups_retained
    );

    // Send system notification on success
    if result.zip_path.is_some() {
        if let Some(ref path) = result.zip_path {
            notify_backup_completed(&app, &profile_name, &path.to_string_lossy());
        }
    }

    result
}

// Re-export server_control commands for convenience
pub use server_control::{get_console_buffer, get_server_status, start_server, stop_server};

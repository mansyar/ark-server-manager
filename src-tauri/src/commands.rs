//! Profile-related Tauri commands.

use crate::models::{Profile, ProfileMetadata};
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
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
        file.read_to_string(&mut json_contents).map_err(|e| e.to_string())?;

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

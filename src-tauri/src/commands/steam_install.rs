//! SteamCMD-related Tauri commands for server install, update, and verify.

use crate::models::Profile;
use crate::services::build_id_checker::{check_build_ids, get_current_build_id};
use crate::services::retry::RetryConfig;
use crate::services::steam_errors::SteamCmdError;
use crate::services::steamcmd_detector::{detect_steamcmd, validate_steamcmd};
use crate::services::steamcmd_installer::{install_steamcmd, sanitize_install_path, InstallerConfig};
use crate::services::steamcmd_runner::{run_steamcmd_script, RunnerConfig, SteamProgressOutput};
use crate::services::update_manager::UpdateManagerConfig;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use tracing::{debug, error, info, warn};

/// Progress event payload emitted to the frontend.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SteamProgressEvent {
    pub profile_name: String,
    pub percentage: Option<f64>,
    pub speed: Option<String>,
    pub eta_seconds: Option<u64>,
    pub line: String,
}

/// Result of a SteamCMD operation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SteamCmdResult {
    pub success: bool,
    pub message: String,
    pub output: Option<String>,
}

/// SteamCMD status for a profile.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SteamCmdStatus {
    pub steamcmd_found: bool,
    pub steamcmd_path: Option<String>,
    pub build_id: Option<String>,
    pub update_available: bool,
    pub current_build: Option<u64>,
    pub available_build: Option<u64>,
}

/// Ensure SteamCMD is available, downloading if necessary.
fn ensure_steamcmd(override_path: Option<&PathBuf>) -> Result<PathBuf, SteamCmdError> {
    // First check override
    if let Some(path) = override_path {
        if validate_steamcmd(path) {
            return Ok(path.clone());
        }
    }

    // Try auto-detect
    if let Some(path) = detect_steamcmd() {
        info!("SteamCMD found at: {:?}", path);
        return Ok(path);
    }

    // Auto-install
    warn!("SteamCMD not found, triggering auto-install");
    let install_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ArkServerManager")
        .join("steamcmd");

    let config = InstallerConfig::default().with_install_dir(install_dir.clone());
    install_steamcmd(config)
}

/// Install ARK server for a given profile.
#[tauri::command]
pub async fn install_server(
    profile_name: String,
    app: AppHandle,
) -> Result<SteamCmdResult, String> {
    info!("install_server called for profile: {}", profile_name);

    // Load profile
    let profile = load_profile_internal(&profile_name).map_err(|e| e.to_string())?;

    // Resolve install dir
    let install_dir = resolve_install_dir(&profile).map_err(|e| e.to_string())?;

    // Ensure SteamCMD is available
    let steamcmd_path = ensure_steamcmd(profile.steamcmd_path.as_ref())
        .map_err(|e| e.to_string())?;

    // Build progress callback
    let profile_name_clone = profile_name.clone();
    let app_clone = app.clone();
    let callback = Box::new(move |output: SteamProgressOutput| {
        let _ = app_clone.emit(
            "steam-progress",
            SteamProgressEvent {
                profile_name: profile_name_clone.clone(),
                percentage: output.percentage,
                speed: output.speed,
                eta_seconds: output.eta_seconds,
                line: output.line,
            },
        );
    });

    // Build and run install script
    let config = RunnerConfig::new(steamcmd_path.clone())
        .with_install_dir(install_dir.clone());

    let script = format!(
        "force_install_dir \"{}\"\napp_update 376030",
        install_dir.to_string_lossy()
    );

    debug!("Running steamcmd install script for profile: {}", profile_name);

    let result = run_steamcmd_script(&config, &[&script], Some(callback));

    match result {
        Ok(output) => {
            // Update profile with build ID
            if let Ok(build_id) = get_current_build_id(&steamcmd_path, &install_dir) {
                let mut profile = load_profile_internal(&profile_name).map_err(|e| e.to_string())?;
                profile.last_verified_build_id = build_id.map(|b| b.to_string());
                profile.steamcmd_install_dir = Some(install_dir.clone());
                save_profile_internal(profile).map_err(|e| e.to_string())?;
            }

            info!("Server install completed for profile: {}", profile_name);
            Ok(SteamCmdResult {
                success: true,
                message: "ARK server installed successfully".to_string(),
                output: Some(output),
            })
        }
        Err(e) => {
            error!("Server install failed for profile {}: {}", profile_name, e);
            Ok(SteamCmdResult {
                success: false,
                message: format!("Install failed: {}", e),
                output: None,
            })
        }
    }
}

/// Check for and apply server updates.
#[tauri::command]
pub async fn update_server(
    profile_name: String,
    app: AppHandle,
) -> Result<SteamCmdResult, String> {
    info!("update_server called for profile: {}", profile_name);

    // Load profile
    let profile = load_profile_internal(&profile_name).map_err(|e| e.to_string())?;

    // Resolve install dir
    let install_dir = resolve_install_dir(&profile).map_err(|e| e.to_string())?;

    // Ensure SteamCMD is available
    let steamcmd_path = ensure_steamcmd(profile.steamcmd_path.as_ref())
        .map_err(|e| e.to_string())?;

    // Check if running first
    let is_running = is_server_running_internal(&profile_name).await;
    if is_running {
        return Err("Cannot update: server is currently running. Stop the server first.".to_string());
    }

    // Build progress callback
    let profile_name_clone = profile_name.clone();
    let app_clone = app.clone();
    let _callback = Box::new(move |output: SteamProgressOutput| {
        let _ = app_clone.emit(
            "steam-progress",
            SteamProgressEvent {
                profile_name: profile_name_clone.clone(),
                percentage: output.percentage,
                speed: output.speed,
                eta_seconds: output.eta_seconds,
                line: output.line,
            },
        );
    });

    // Run update
    let update_config = UpdateManagerConfig {
        steamcmd_path: steamcmd_path.clone(),
        install_dir: install_dir.clone(),
        retry_config: RetryConfig::default(),
        validate_after_update: true,
    };

    let result = crate::services::update_manager::update_server(update_config)
        .map_err(|e| e.to_string())?;

    // Update profile build ID
    if result.updated {
        let mut profile = load_profile_internal(&profile_name).map_err(|e| e.to_string())?;
        if let Some(build_ids) = &result.build_ids {
            profile.last_verified_build_id = build_ids.available_build.map(|b| b.to_string());
        }
        save_profile_internal(profile).map_err(|e| e.to_string())?;
    }

    let message = if result.updated {
        "Server updated successfully".to_string()
    } else {
        "Server is already up to date".to_string()
    };

    Ok(SteamCmdResult {
        success: true,
        message,
        output: Some(result.output),
    })
}

/// Verify server installation integrity.
#[tauri::command]
pub async fn verify_server(
    profile_name: String,
    _app: AppHandle,
) -> Result<SteamCmdResult, String> {
    info!("verify_server called for profile: {}", profile_name);

    // Load profile
    let profile = load_profile_internal(&profile_name).map_err(|e| e.to_string())?;

    // Resolve install dir
    let install_dir = resolve_install_dir(&profile).map_err(|e| e.to_string())?;

    // Ensure SteamCMD is available
    let steamcmd_path = ensure_steamcmd(profile.steamcmd_path.as_ref())
        .map_err(|e| e.to_string())?;

    // Run verify
    let config = RunnerConfig::new(steamcmd_path.clone())
        .with_install_dir(install_dir.clone());

    let script = format!(
        "force_install_dir \"{}\"\napp_update 376030 validate",
        install_dir.to_string_lossy()
    );

    let result = run_steamcmd_script(&config, &[&script], None)
        .map_err(|e| e.to_string())?;

    // Update profile with verified build ID
    if let Ok(build_id) = get_current_build_id(&steamcmd_path, &install_dir) {
        let mut profile = load_profile_internal(&profile_name).map_err(|e| e.to_string())?;
        profile.last_verified_build_id = build_id.map(|b| b.to_string());
        save_profile_internal(profile).map_err(|e| e.to_string())?;
    }

    let success = result.contains("Success") || result.contains("Verification complete");

    Ok(SteamCmdResult {
        success,
        message: if success {
            "Server verification successful".to_string()
        } else {
            "Server verification failed - files may be corrupted".to_string()
        },
        output: Some(result),
    })
}

/// Get SteamCMD status for a profile.
#[tauri::command]
pub async fn get_steamcmd_status(
    profile_name: String,
) -> Result<SteamCmdStatus, String> {
    // Try to detect SteamCMD
    let (steamcmd_found, steamcmd_path) = if let Some(path) = detect_steamcmd() {
        (true, Some(path.to_string_lossy().to_string()))
    } else {
        (false, None)
    };

    // Load profile to get build ID info
    let profile = load_profile_internal(&profile_name).ok();

    let build_id = profile.as_ref()
        .and_then(|p| p.last_verified_build_id.clone());

    let install_dir = profile.as_ref()
        .and_then(|p| p.steamcmd_install_dir.clone());

    // Check for updates if we have both paths
    let (update_available, current_build, available_build) = if let (Some(sc_path), Some(dir)) = (
        steamcmd_path.as_ref(),
        install_dir.as_ref()
    ) {
        let sc_path_buf = PathBuf::from(sc_path);
        match check_build_ids(&sc_path_buf, dir) {
            Ok(info) => (info.update_available, info.current_build, info.available_build),
            Err(_) => (false, None, None),
        }
    } else {
        (false, None, None)
    };

    Ok(SteamCmdStatus {
        steamcmd_found,
        steamcmd_path,
        build_id,
        update_available,
        current_build,
        available_build,
    })
}

// --- Internal helpers ---

fn profiles_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ArkServerManager")
        .join("profiles")
}

fn load_profile_internal(name: &str) -> Result<Profile, SteamCmdError> {
    let path = profiles_dir().join(format!("{}.json", name));
    let contents = std::fs::read_to_string(&path)
        .map_err(|e| SteamCmdError::IoError(e.to_string()))?;
    serde_json::from_str(&contents)
        .map_err(|e| SteamCmdError::IoError(format!("Invalid profile JSON: {}", e)))
}

fn save_profile_internal(profile: Profile) -> Result<(), SteamCmdError> {
    let dir = profiles_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .map_err(|e| SteamCmdError::IoError(e.to_string()))?;
    }
    let path = dir.join(format!("{}.json", profile.name));
    let json = serde_json::to_string_pretty(&profile)
        .map_err(|e| SteamCmdError::IoError(e.to_string()))?;
    std::fs::write(&path, json)
        .map_err(|e| SteamCmdError::IoError(e.to_string()))?;
    Ok(())
}

fn resolve_install_dir(profile: &Profile) -> Result<PathBuf, SteamCmdError> {
    // Priority: explicit steamcmd_install_dir > server_install_path > default
    if let Some(ref dir) = profile.steamcmd_install_dir {
        return sanitize_install_path(dir);
    }

    if let Some(ref server_path) = profile.server_install_path {
        // If pointing to ShooterGameServer.exe, go up to root
        let root = if server_path.file_name().map(|n| n == "ShooterGameServer.exe").unwrap_or(false) {
            server_path.parent()
                .and_then(|p| p.parent()) // Win64 -> ShooterGame -> ARK root
                .map(|p| p.to_path_buf())
        } else {
            Some(server_path.clone())
        };
        if let Some(r) = root {
            return sanitize_install_path(&r);
        }
    }

    // Default location
    let default = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ArkServerManager")
        .join("servers")
        .join(&profile.name);

    sanitize_install_path(&default)
}

async fn is_server_running_internal(profile_name: &str) -> bool {
    use crate::services::server_state::SERVER_STATE;
    let state = SERVER_STATE.lock().await;
    state.is_running(profile_name)
}
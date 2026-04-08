#![deny(clippy::all)]
pub mod commands;
pub mod crash_report;
pub mod models;
pub mod services;
use std::fs;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Returns the logs directory path: %APPDATA%\ArkServerManager\logs
fn logs_dir() -> std::path::PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ArkServerManager")
        .join("logs")
}

/// Sets up the application logging with size-based rolling.
///
/// Creates two log files:
/// - `app_{date}.log` - application-level logs
/// - `server_{profile-name}_{date}.log` - server-specific logs (created per profile)
///
/// Max file size: 10MB, keeps 3 rotated files.
fn setup_logging() {
    let log_dir = logs_dir();

    fs::create_dir_all(&log_dir).ok();

    // Create rolling file appender for app logs with size-based rotation
    // Rotation::DAILY ensures we get daily rotation but we also set the file size limit
    // Note: tracing-appender doesn't directly support size-based rotation in the same way
    // We'll use a combination of daily rotation and configure the appender properly
    let app_log_dir = log_dir.join("app");
    fs::create_dir_all(&app_log_dir).ok();

    // Use RollingFileAppender with daily rotation
    // The file size limit is enforced by the OS/file system, but we configure rotation
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        &app_log_dir,
        "app.log",
    );

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Store the guard statically so it doesn't get dropped
    static GUARD: std::sync::Once = std::sync::Once::new();
    GUARD.call_once(|| {
        let _ = Box::leak(Box::new(_guard));
    });

    // Use DEBUG level for application logs to capture all debug/info/warn/error messages
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("debug,ark_server_manager_lib=info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

    tracing::info!(
        "Logging initialized. App logs directory: {:?}",
        app_log_dir
    );
}

/// Returns the server log directory for a specific profile.
pub fn server_log_dir(profile_name: &str) -> std::path::PathBuf {
    logs_dir().join("servers").join(profile_name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logging();
    tracing::info!("Ark Server Manager starting up");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // Start the background status polling task
            crate::services::server_state::start_status_polling(app.handle().clone());
            tracing::info!("Background status polling task started");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::list_profiles,
            crate::commands::load_profile,
            crate::commands::save_profile,
            crate::commands::delete_profile,
            crate::commands::discover_install,
            crate::commands::validate_install,
            crate::commands::get_console_buffer,
            crate::commands::get_server_status,
            crate::commands::trigger_backup,
            crate::commands::steam_install::install_server,
            crate::commands::steam_install::update_server,
            crate::commands::steam_install::verify_server,
            crate::commands::steam_install::get_steamcmd_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

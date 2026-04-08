//! Notification service for system notifications.
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

/// Sends a system notification.
///
/// Falls back silently if notifications are not permitted or fail.
pub fn notify(app: &AppHandle, title: &str, body: &str) {
    let _ = app.notification().builder().title(title).body(body).show();
}

/// Sends a server started notification.
pub fn notify_server_started(app: &AppHandle, profile_name: &str) {
    notify(
        app,
        "ARK Server Started",
        &format!("Server '{}' is now running.", profile_name),
    );
}

/// Sends a server stopped notification.
pub fn notify_server_stopped(app: &AppHandle, profile_name: &str) {
    notify(
        app,
        "ARK Server Stopped",
        &format!("Server '{}' has been stopped.", profile_name),
    );
}

/// Sends a backup completed notification.
pub fn notify_backup_completed(app: &AppHandle, profile_name: &str, backup_path: &str) {
    notify(
        app,
        "ARK Backup Complete",
        &format!(
            "Backup for '{}' completed successfully.\nSaved to: {}",
            profile_name, backup_path
        ),
    );
}

/// Sends a crash detected notification.
pub fn notify_crash_detected(app: &AppHandle, profile_name: &str) {
    notify(
        app,
        "ARK Server Crashed",
        &format!(
            "Server '{}' has crashed. A crash report has been saved.",
            profile_name
        ),
    );
}

pub mod backup;
pub mod build_id_checker;
pub mod health;
pub mod ini_conversion;
pub mod rcon_client;
pub mod retry;
pub mod server_discovery;
pub mod server_state;
pub mod steam_errors;
pub mod steam_progress;
pub mod steamcmd_detector;
pub mod steamcmd_installer;
pub mod steamcmd_runner;
pub mod update_manager;

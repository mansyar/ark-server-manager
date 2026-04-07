#![deny(clippy::all)]
pub mod commands;
pub mod models;
pub mod services;
use std::fs;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn setup_logging() {
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ArkServerManager")
        .join("logs");

    fs::create_dir_all(&log_dir).ok();

    let file_appender = tracing_appender::rolling::daily(&log_dir, "asm.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Store the guard statically so it doesn't get dropped
    static GUARD: std::sync::Once = std::sync::Once::new();
    GUARD.call_once(|| {
        let _ = Box::leak(Box::new(_guard));
    });

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_logging();
    tracing::info!("Ark Server Manager starting up");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Schema version for profile JSON files.
/// Bump this when breaking changes require migration.
pub const PROFILE_SCHEMA_VERSION: u32 = 1;

/// Profile metadata returned by `list_profiles`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileMetadata {
    pub name: String,
    pub map: String,
    pub last_modified: std::time::SystemTime,
}

/// Server profile representing a single ARK server installation.
///
/// Stored as JSON at `%APPDATA%\ArkServerManager\profiles\{name}.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Schema version for future migrations.
    pub schema_version: u32,

    /// Display name of the profile.
    pub name: String,

    /// ARK map to use (e.g., "TheIsland", "ScorchedEarth", " Ragnarot").
    pub map: String,

    /// Difficulty multiplier (0.0 – 20.0, typically integer 0–10 for official).
    pub difficulty: f64,

    /// Maximum number of players allowed.
    pub max_players: u32,

    /// Admin password — stored in GameUserSettings.ini.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,

    /// Query port for the server (27000–27015).
    pub port: u32,

    /// Additional raw key/value pairs not covered by the typed fields.
    /// Stored under `[ServerSettings]` in Game.ini.
    #[serde(default)]
    pub extra_settings: HashMap<String, String>,

    /// Raw GameUserSettings.ini key/value pairs not covered by typed fields.
    /// Stored under `[GameUserSettings]` in GameUserSettings.ini.
    #[serde(default)]
    pub extra_user_settings: HashMap<String, String>,

    /// Optional path to a custom ARK server installation.
    /// If set, this overrides the default discovery path.
    /// Can point to ShooterGameServer.exe directly or to the ARK install root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_install_path: Option<std::path::PathBuf>,

    /// Optional path to SteamCMD binary for this profile.
    /// If set, this overrides the global SteamCMD detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steamcmd_path: Option<std::path::PathBuf>,

    /// Last verified build ID for this profile's server installation.
    /// Updated after install, update, or verify operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_verified_build_id: Option<String>,

    /// Directory where SteamCMD and ARK server files are installed for this profile.
    /// Should point to the root ARK server install directory (contains ShooterGame/Binaries/Win64).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steamcmd_install_dir: Option<std::path::PathBuf>,

    /// Directory where backups for this profile should be stored.
    /// If None, defaults to `{steamcmd_install_dir}/backups`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_dir: Option<std::path::PathBuf>,

    /// Maximum number of backups to retain for this profile.
    /// When a new backup is created and this limit is exceeded,
    /// the oldest backup(s) are deleted. Set to 0 to disable retention enforcement.
    #[serde(default)]
    pub backup_retention_count: u32,

    /// Filename suffix used for backup ZIP files.
    /// The pattern is `{profile_name}_{timestamp}.zip`.
    /// Defaults to "backup".
    #[serde(default = "default_backup_suffix")]
    pub backup_suffix: String,

    /// Whether to automatically restart the server on crash.
    /// Defaults to true.
    #[serde(default = "default_auto_restart_on_crash")]
    pub auto_restart_on_crash: bool,

    /// Delay in seconds before attempting auto-restart after crash.
    /// Defaults to 10 seconds. Valid range: 5-60 seconds.
    #[serde(default = "default_auto_restart_delay_secs")]
    pub auto_restart_delay_secs: u32,

    /// Maximum number of restart attempts within 5 minutes before giving up.
    /// Defaults to 3.
    #[serde(default = "default_max_restart_attempts")]
    pub max_restart_attempts: u32,
}

fn default_backup_suffix() -> String {
    "backup".to_string()
}

fn default_auto_restart_on_crash() -> bool {
    true
}

fn default_auto_restart_delay_secs() -> u32 {
    10
}

fn default_max_restart_attempts() -> u32 {
    3
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            schema_version: PROFILE_SCHEMA_VERSION,
            name: String::new(),
            map: "TheIsland".to_string(),
            difficulty: 1.0,
            max_players: 70,
            admin_password: None,
            port: 27015,
            extra_settings: HashMap::new(),
            extra_user_settings: HashMap::new(),
            server_install_path: None,
            steamcmd_path: None,
            last_verified_build_id: None,
            steamcmd_install_dir: None,
            backup_dir: None,
            backup_retention_count: 0,
            backup_suffix: "backup".to_string(),
            auto_restart_on_crash: true,
            auto_restart_delay_secs: 10,
            max_restart_attempts: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_default() {
        let p = Profile::default();
        assert_eq!(p.schema_version, PROFILE_SCHEMA_VERSION);
        assert_eq!(p.map, "TheIsland");
        assert_eq!(p.difficulty, 1.0);
        assert_eq!(p.max_players, 70);
        assert_eq!(p.port, 27015);
    }

    #[test]
    fn test_profile_serde_roundtrip() {
        let p = Profile {
            name: "TestProfile".to_string(),
            map: "ScorchedEarth".to_string(),
            difficulty: 5.0,
            max_players: 10,
            admin_password: Some("secret".to_string()),
            port: 27000,
            extra_settings: [("ActiveMods".to_string(), "mod1,mod2".to_string())]
                .into_iter()
                .collect(),
            ..Default::default()
        };

        let json = serde_json::to_string(&p).unwrap();
        let parsed: Profile = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, p.name);
        assert_eq!(parsed.map, p.map);
        assert_eq!(parsed.difficulty, p.difficulty);
        assert_eq!(parsed.max_players, p.max_players);
        assert_eq!(parsed.admin_password, p.admin_password);
        assert_eq!(parsed.port, p.port);
        assert_eq!(
            parsed.extra_settings.get("ActiveMods"),
            p.extra_settings.get("ActiveMods")
        );
    }
}

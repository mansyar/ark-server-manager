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

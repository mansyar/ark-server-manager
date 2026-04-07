//! INI file conversion for ARK server profiles.
//!
//! ARK stores settings in two INI files:
//! - `Game.ini` — server settings (map, difficulty, max players)
//! - `GameUserSettings.ini` — user-facing settings (admin password, port, etc.)

use crate::models::Profile;
use configparser::ini::Ini;
use std::collections::HashMap;
use std::io::Write;
use tracing::{error, info, warn};

/// Key constants for ARK INI sections.
pub const SECTION_SERVER_SETTINGS: &str = "/script/shootergame/shootreplicationlibrary";
pub const SECTION_SESSION_NAME: &str = "SessionName";
pub const SECTION_GAME_USER_SETTINGS: &str = "GameUserSettings";
pub const SECTION_AKSETTINGS: &str = "Aksettings";

/// Converts an ARK INI string (Game.ini or GameUserSettings.ini) into a `Profile`.
///
/// `ini_type` should be `"game"` for Game.ini or `"user"` for GameUserSettings.ini.
pub fn ini_to_profile(
    ini_contents: &str,
    ini_type: &str,
    base_profile: Option<&Profile>,
) -> Result<Profile, String> {
    let ini = Ini::load_from_str(ini_contents).map_err(|e| format!("Failed to parse INI: {}", e))?;

    let mut profile = base_profile.cloned().unwrap_or_default();
    profile.schema_version = crate::models::PROFILE_SCHEMA_VERSION;

    if ini_type == "game" {
        // --- Game.ini ---
        if let Some(section) = ini.section(Some(SECTION_SERVER_SETTINGS)) {
            if let Some(val) = section.get("DifficultyOffset") {
                if let Ok(v) = val.parse::<f64>() {
                    profile.difficulty = v;
                }
            }
            if let Some(val) = section.get("MaxPlayers") {
                if let Ok(v) = val.parse::<u32>() {
                    profile.max_players = v;
                }
            }
            if let Some(val) = section.get("Akset_MapName") {
                profile.map = val.to_string();
            }
        }

        // Extra raw key/values from Game.ini
        if let Some(section) = ini.section(Some(SECTION_SERVER_SETTINGS)) {
            for (k, v) in section.iter() {
                if !["DifficultyOffset", "MaxPlayers", "Akset_MapName"].contains(&k.as_str()) {
                    profile.extra_settings.insert(k.to_string(), v.to_string());
                }
            }
        }
    } else {
        // --- GameUserSettings.ini ---
        if let Some(section) = ini.section(Some(SECTION_GAME_USER_SETTINGS)) {
            if let Some(val) = section.get("AdminPassword") {
                profile.admin_password = Some(val.to_string());
            }
            if let Some(val) = section.get("QueryPort") {
                if let Ok(v) = val.parse::<u32>() {
                    profile.port = v;
                }
            }
        }

        // Extra raw key/values from GameUserSettings.ini
        if let Some(section) = ini.section(Some(SECTION_GAME_USER_SETTINGS)) {
            for (k, v) in section.iter() {
                if !["AdminPassword", "QueryPort"].contains(&k.as_str()) {
                    profile
                        .extra_user_settings
                        .insert(k.to_string(), v.to_string());
                }
            }
        }
    }

    Ok(profile)
}

/// Converts a `Profile` into a Game.ini INI string.
pub fn profile_to_game_ini(profile: &Profile) -> String {
    let mut ini = Ini::new();
    let section_name = SECTION_SERVER_SETTINGS;

    // Core fields
    ini.with_section(section_name)
        .set("DifficultyOffset", profile.difficulty.to_string())
        .set("MaxPlayers", profile.max_players.to_string())
        .set("Akset_MapName", &profile.map);

    // Extra raw settings
    for (k, v) in &profile.extra_settings {
        ini.with_section(section_name).set(k, v);
    }

    // Write to string
    let mut output = Vec::new();
    ini.write_to(&mut output)
        .map_err(|e| format!("Failed to write INI: {}", e))?;
    String::from_utf8(output).map_err(|e| format!("INI output not UTF-8: {}", e))
}

/// Converts a `Profile` into a GameUserSettings.ini INI string.
pub fn profile_to_game_user_settings_ini(profile: &Profile) -> String {
    let mut ini = Ini::new();
    let section_name = SECTION_GAME_USER_SETTINGS;

    if let Some(ref pw) = profile.admin_password {
        if !pw.is_empty() {
            ini.with_section(section_name).set("AdminPassword", pw);
        }
    }
    ini.with_section(section_name)
        .set("QueryPort", profile.port.to_string());

    // Extra raw user settings
    for (k, v) in &profile.extra_user_settings {
        ini.with_section(section_name).set(k, v);
    }

    // Write to string
    let mut output = Vec::new();
    ini.write_to(&mut output)
        .map_err(|e| format!("Failed to write INI: {}", e))?;
    String::from_utf8(output).map_err(|e| format!("INI output not UTF-8: {}", e))
}

/// Reads both INI files from a server install directory and merges them into a `Profile`.
pub fn read_profile_from_ini_dir(
    game_ini_path: &std::path::Path,
    game_user_settings_ini_path: &std::path::Path,
) -> Result<Profile, String> {
    let base = Profile::default();

    // Parse Game.ini
    if game_ini_path.exists() {
        let contents =
            std::fs::read_to_string(game_ini_path).map_err(|e| format!("Failed to read Game.ini: {}", e))?;
        base.name = std::path::Path::new(game_ini_path)
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Unnamed")
            .to_string();
        base.name = sanitize_name_for_filename(&base.name);
        let parsed = ini_to_profile(&contents, "game", Some(&base))?;
        let mut profile = parsed;

        // Parse GameUserSettings.ini on top
        if game_user_settings_ini_path.exists() {
            let contents = std::fs::read_to_string(game_user_settings_ini_path)
                .map_err(|e| format!("Failed to read GameUserSettings.ini: {}", e))?;
            profile = ini_to_profile(&contents, "user", Some(&profile))?;
        }
        info!("Loaded profile '{}' from INI directory", profile.name);
        return Ok(profile);
    }

    Err("Neither Game.ini nor GameUserSettings.ini found".to_string())
}

/// Sanitize a profile name to be filename-safe.
fn sanitize_name_for_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_to_game_ini() {
        let mut profile = Profile::default();
        profile.name = "TestServer".to_string();
        profile.map = "ScorchedEarth".to_string();
        profile.difficulty = 3.0;
        profile.max_players = 50;

        let ini_str = profile_to_game_ini(&profile);
        let ini = Ini::load_from_str(&ini_str).unwrap();

        let section = ini.section(Some(SECTION_SERVER_SETTINGS)).unwrap();
        assert_eq!(section.get("DifficultyOffset"), Some(&"3".to_string()));
        assert_eq!(section.get("MaxPlayers"), Some(&"50".to_string()));
        assert_eq!(section.get("Akset_MapName"), Some(&"ScorchedEarth".to_string()));
    }

    #[test]
    fn test_profile_to_game_user_settings_ini() {
        let mut profile = Profile::default();
        profile.name = "TestServer".to_string();
        profile.admin_password = Some("hunter2".to_string());
        profile.port = 27015;

        let ini_str = profile_to_game_user_settings_ini(&profile);
        let ini = Ini::load_from_str(&ini_str).unwrap();

        let section = ini.section(Some(SECTION_GAME_USER_SETTINGS)).unwrap();
        assert_eq!(section.get("AdminPassword"), Some(&"hunter2".to_string()));
        assert_eq!(section.get("QueryPort"), Some(&"27015".to_string()));
    }

    #[test]
    fn test_ini_to_profile_roundtrip_game() {
        let mut profile = Profile::default();
        profile.name = "RoundtripTest".to_string();
        profile.map = "Ragnarok".to_string();
        profile.difficulty = 7.0;
        profile.max_players = 80;

        let ini_str = profile_to_game_ini(&profile);
        let parsed = ini_to_profile(&ini_str, "game", None).unwrap();

        assert_eq!(parsed.difficulty, 7.0);
        assert_eq!(parsed.max_players, 80);
        assert_eq!(parsed.map, "Ragnarok".to_string());
    }

    #[test]
    fn test_ini_to_profile_roundtrip_user() {
        let mut profile = Profile::default();
        profile.name = "RoundtripTest".to_string();
        profile.admin_password = Some("securepass".to_string());
        profile.port = 27010;

        let ini_str = profile_to_game_user_settings_ini(&profile);
        let parsed = ini_to_profile(&ini_str, "user", None).unwrap();

        assert_eq!(parsed.admin_password, Some("securepass".to_string()));
        assert_eq!(parsed.port, 27010);
    }
}

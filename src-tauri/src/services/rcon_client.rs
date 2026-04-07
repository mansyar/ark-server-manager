//! RCON client for ARK server remote administration.
//!
//! Provides async RCON connection to ARK servers via the standard RCON protocol.
//! ARK RCON port is QueryPort + 1 (ARK convention).

use chrono::{DateTime, Utc};
use rcon::Error as RconError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::debug;

/// RCON connection wrapper for ARK server remote administration.
/// Wraps the async rcon crate's Connection with tokio TcpStream.
#[derive(Clone)]
pub struct ArkRconClient {
    /// Profile name this client is connected to.
    profile_name: String,
    /// Internal RCON connection wrapped in mutex for safe concurrent access.
    conn: Arc<Mutex<rcon::Connection<TcpStream>>>,
    /// The query port used to derive RCON port (QueryPort + 1).
    query_port: u16,
}

impl ArkRconClient {
    /// Creates a new ARK RCON client connected to the specified server.
    ///
    /// # Arguments
    ///
    /// * `profile_name` - Name of the profile for the server
    /// * `addr` - Socket address of the RCON server (host, port)
    /// * `password` - RCON password for authentication
    ///
    /// # Returns
    ///
    /// A connected and authenticated RCON client, or an error.
    pub async fn new(
        profile_name: String,
        addr: SocketAddr,
        password: &str,
    ) -> Result<Self, RconConnError> {
        debug!("Connecting RCON to {} for profile '{}'", addr, profile_name);

        let conn = rcon::Connection::builder()
            .connect(addr, password)
            .await
            .map_err(|e| RconConnError::ConnectionFailed {
                addr: addr.to_string(),
                reason: e.to_string(),
            })?;

        debug!(
            "RCON authenticated successfully for profile '{}'",
            profile_name
        );

        Ok(Self {
            profile_name,
            conn: Arc::new(Mutex::new(conn)),
            query_port: addr.port().saturating_sub(1), // RCON port = QueryPort + 1
        })
    }

    /// Executes a raw RCON command and returns the response.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The command string to execute
    ///
    /// # Returns
    ///
    /// The response string from the server, or an error.
    pub async fn execute(&self, cmd: &str) -> Result<String, RconConnError> {
        let mut conn = self.conn.lock().await;
        conn.cmd(cmd)
            .await
            .map_err(|e| RconConnError::CommandFailed {
                command: cmd.to_string(),
                reason: e.to_string(),
            })
    }

    /// Gets the profile name this client is connected to.
    pub fn profile_name(&self) -> &str {
        &self.profile_name
    }

    /// Gets the query port for this connection.
    pub fn query_port(&self) -> u16 {
        self.query_port
    }
}

/// RCON connection error types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RconConnError {
    /// Failed to connect to the RCON server.
    ConnectionFailed { addr: String, reason: String },
    /// Authentication with RCON password failed.
    AuthFailed { reason: String },
    /// Command execution failed.
    CommandFailed { command: String, reason: String },
    /// Server not running (no handle found).
    ServerNotRunning { profile_name: String },
    /// Profile not found.
    ProfileNotFound { name: String },
    /// Profile load failed.
    ProfileLoadFailed { name: String, reason: String },
    /// RCON port not available.
    PortNotAvailable { port: u16 },
    /// Timed out waiting for response.
    Timeout { command: String },
}

impl fmt::Display for RconConnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RconConnError::ConnectionFailed { addr, reason } => {
                write!(f, "RCON connection failed to {}: {}", addr, reason)
            }
            RconConnError::AuthFailed { reason } => {
                write!(f, "RCON authentication failed: {}", reason)
            }
            RconConnError::CommandFailed { command, reason } => {
                write!(f, "RCON command '{}' failed: {}", command, reason)
            }
            RconConnError::ServerNotRunning { profile_name } => {
                write!(f, "Server for profile '{}' is not running", profile_name)
            }
            RconConnError::ProfileNotFound { name } => {
                write!(f, "Profile '{}' not found", name)
            }
            RconConnError::ProfileLoadFailed { name, reason } => {
                write!(f, "Failed to load profile '{}': {}", name, reason)
            }
            RconConnError::PortNotAvailable { port } => {
                write!(f, "RCON port {} is not available", port)
            }
            RconConnError::Timeout { command } => {
                write!(f, "RCON command '{}' timed out", command)
            }
        }
    }
}

impl std::error::Error for RconConnError {}

impl From<RconError> for RconConnError {
    fn from(err: RconError) -> Self {
        match err {
            RconError::Auth => RconConnError::AuthFailed {
                reason: "Authentication failed".to_string(),
            },
            RconError::Io(io_err) => RconConnError::ConnectionFailed {
                addr: "unknown".to_string(),
                reason: io_err.to_string(),
            },
            _ => RconConnError::ConnectionFailed {
                addr: "unknown".to_string(),
                reason: err.to_string(),
            },
        }
    }
}

/// Player information retrieved from ARK server via RCON.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerInfo {
    /// Player's display name.
    pub name: String,
    /// ARK player ID.
    pub player_id: u64,
    /// Tribe name (empty string if not in a tribe).
    pub tribe: String,
    /// When the player joined (if available, None otherwise).
    pub joined_at: Option<DateTime<Utc>>,
}

impl fmt::Display for PlayerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (ID:{}) [Tribe:{}]",
            self.name,
            self.player_id,
            if self.tribe.is_empty() {
                "None".to_string()
            } else {
                self.tribe.clone()
            }
        )
    }
}

/// Parses the `PlayerId` command output from ARK server.
///
/// The output format is typically:
/// ```
/// PlayerId list:
/// PlayerName (ID:12345) (Tribe:TribeName)
/// ```
///
/// Some players may not have a tribe (no Tribe: tag).
///
/// # Arguments
///
/// * `output` - Raw output from the PlayerId command
///
/// # Returns
///
/// A vector of parsed `PlayerInfo` entries.
pub fn parse_player_list(output: &str) -> Vec<PlayerInfo> {
    let mut players = Vec::new();

    // Split output into lines
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Skip header lines that aren't player entries
        if line.starts_with("PlayerId list:") || line.starts_with("Total players connected:") {
            continue;
        }

        // ARK PlayerId output format: "PlayerName (ID:12345) (Tribe:TribeName)"
        // or if no tribe: "PlayerName (ID:12345)"
        // Some formats may have additional whitespace or variations.

        // Try to parse the name, ID, and tribe
        if let Some(player) = parse_player_line(line) {
            players.push(player);
        }
    }

    players
}

/// Parses a single player line from the PlayerId output.
///
/// Format: "PlayerName (ID:12345) (Tribe:TribeName)"
/// or: "PlayerName (ID:12345)" (no tribe)
///
/// Returns None if the line doesn't match expected format.
fn parse_player_line(line: &str) -> Option<PlayerInfo> {
    // Regex to parse: Name (ID:12345) (Tribe:TribeName) or Name (ID:12345)
    // The name may contain spaces, parentheses are delimiters.

    // Find the ID portion
    let id_start = line.find("(ID:")?;
    let id_end = line[id_start..].find(')')?;

    let id_str = &line[id_start + 4..id_start + id_end];
    let player_id: u64 = id_str.parse().ok()?;

    // Extract name (everything before "(ID:")
    let name = line[..id_start].trim().to_string();

    // Check for tribe
    let remaining = &line[id_start + id_end + 1..];
    let tribe = if let Some(tribe_start) = remaining.find("(Tribe:") {
        let tribe_content = &remaining[tribe_start + 7..];
        let tribe_end = tribe_content.find(')').unwrap_or(tribe_content.len());
        tribe_content[..tribe_end].to_string()
    } else {
        String::new()
    };

    Some(PlayerInfo {
        name,
        player_id,
        tribe,
        joined_at: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_player_list_basic() {
        let output = r#"PlayerId list:
PlayerName1 (ID:12345) (Tribe:TribeName1)
PlayerName2 (ID:67890) (Tribe:)
PlayerName3 (ID:11111)
Total players connected: 3"#;

        let players = parse_player_list(output);

        assert_eq!(players.len(), 3);
        assert_eq!(players[0].name, "PlayerName1");
        assert_eq!(players[0].player_id, 12345);
        assert_eq!(players[0].tribe, "TribeName1");
        assert_eq!(players[1].name, "PlayerName2");
        assert_eq!(players[1].player_id, 67890);
        assert_eq!(players[1].tribe, "");
        assert_eq!(players[2].name, "PlayerName3");
        assert_eq!(players[2].player_id, 11111);
        assert_eq!(players[2].tribe, "");
    }

    #[test]
    fn test_parse_player_list_empty_tribe() {
        let output = "PlayerName (ID:12345) (Tribe:)";
        let players = parse_player_list(output);
        assert_eq!(players.len(), 1);
        assert_eq!(players[0].tribe, "");
    }

    #[test]
    fn test_parse_player_list_no_tribe() {
        let output = "PlayerName (ID:12345)";
        let players = parse_player_list(output);
        assert_eq!(players.len(), 1);
        assert_eq!(players[0].name, "PlayerName");
        assert_eq!(players[0].player_id, 12345);
        assert_eq!(players[0].tribe, "");
    }

    #[test]
    fn test_parse_player_list_empty_output() {
        let output = "";
        let players = parse_player_list(output);
        assert!(players.is_empty());
    }

    #[test]
    fn test_parse_player_list_header_lines_only() {
        let output = "PlayerId list:\nTotal players connected: 0";
        let players = parse_player_list(output);
        assert!(players.is_empty());
    }

    #[test]
    fn test_parse_player_line_with_spaces_in_name() {
        let line = "John Doe (ID:12345) (Tribe:Awesome Tribe)";
        let players = parse_player_list(line);
        assert_eq!(players.len(), 1);
        assert_eq!(players[0].name, "John Doe");
        assert_eq!(players[0].player_id, 12345);
        assert_eq!(players[0].tribe, "Awesome Tribe");
    }

    #[test]
    fn test_parse_player_line_invalid() {
        let line = "Invalid line format";
        let players = parse_player_list(line);
        assert!(players.is_empty());
    }

    #[test]
    fn test_parse_player_line_no_id() {
        let line = "PlayerName (NoID)";
        let players = parse_player_list(line);
        assert!(players.is_empty());
    }

    #[test]
    fn test_parse_player_line_invalid_id() {
        let line = "PlayerName (ID:abc)";
        let players = parse_player_list(line);
        assert!(players.is_empty());
    }

    #[test]
    fn test_player_info_display() {
        let player = PlayerInfo {
            name: "TestPlayer".to_string(),
            player_id: 12345,
            tribe: "TestTribe".to_string(),
            joined_at: None,
        };
        assert_eq!(
            format!("{}", player),
            "TestPlayer (ID:12345) [Tribe:TestTribe]"
        );
    }

    #[test]
    fn test_player_info_display_no_tribe() {
        let player = PlayerInfo {
            name: "TestPlayer".to_string(),
            player_id: 12345,
            tribe: "".to_string(),
            joined_at: None,
        };
        assert_eq!(format!("{}", player), "TestPlayer (ID:12345) [Tribe:None]");
    }

    #[test]
    fn test_parse_player_list_with_whitespace() {
        let output =
            "  PlayerName1 (ID:12345) (Tribe:TribeName1)  \n  \n  PlayerName2 (ID:67890)  ";
        let players = parse_player_list(output);
        assert_eq!(players.len(), 2);
    }

    #[test]
    fn test_parse_player_list_tribe_with_spaces() {
        let output = "PlayerName (ID:12345) (Tribe:My Awesome Tribe)";
        let players = parse_player_list(output);
        assert_eq!(players.len(), 1);
        assert_eq!(players[0].tribe, "My Awesome Tribe");
    }

    #[test]
    fn test_parse_player_list_tribe_empty_after_colon() {
        let output = "PlayerName (ID:12345) (Tribe:)";
        let players = parse_player_list(output);
        assert_eq!(players.len(), 1);
        assert_eq!(players[0].tribe, "");
    }

    #[test]
    fn test_rcon_error_display() {
        let err = RconConnError::ConnectionFailed {
            addr: "127.0.0.1:27016".to_string(),
            reason: "Connection refused".to_string(),
        };
        assert_eq!(
            format!("{}", err),
            "RCON connection failed to 127.0.0.1:27016: Connection refused"
        );
    }

    #[test]
    fn test_rcon_error_from_auth() {
        let rcon_err = RconError::Auth;
        let err = RconConnError::from(rcon_err);
        match err {
            RconConnError::AuthFailed { reason } => {
                assert!(reason.contains("Authentication failed"));
            }
            _ => panic!("Expected AuthFailed variant"),
        }
    }

    #[test]
    fn test_parse_player_line_malformed_tribe() {
        // Tribe without closing paren
        let line = "PlayerName (ID:12345) (Tribe:TestTribe";
        let players = parse_player_list(line);
        assert_eq!(players.len(), 1);
        // It will extract tribe until where it finds the closing paren
        // but since there's no closing paren, it should still get TestTribe
        assert_eq!(players[0].tribe, "TestTribe");
    }
}

# Phase 2 — Server Lifecycle Management: Implementation Plan

## Phase 2.1: Server Discovery & Path Management

- [ ] **2.1.1** Add `ServerInstall` struct to Rust backend: `{ steamcmd_path: PathBuf, ark_exe_path: PathBuf, install_dir: PathBuf }`
- [ ] **2.1.2** Implement `discover_server_install() -> Result<ServerInstall, DiscoveryError>` with default paths:
  - `C:\Program Files (x86)\Steam\steamapps\common\ARK\ShooterGame\Binaries\Win64\ShooterGameServer.exe`
  - `C:\Program Files (x86)\Steam\steamapps\common\ARK\Engine\Binaries\ThirdParty\SteamCMD\steamcmd.exe`
- [ ] **2.1.3** Validate both binaries exist; return `InstallNotFound` with guidance if missing
- [ ] **2.1.4** Add Tauri command `discover_install() -> ServerInstall` for frontend
- [ ] **2.1.5** Add `Profile` field: `server_install_path: Option<PathBuf>` (user override)
- [ ] **2.1.6** Add `validate_install(profile_name: String) -> ValidationResult` (checks exe exists)
- [ ] **2.1.7** Run `cargo clippy --all-targets --all-features` — zero warnings

## Phase 2.2: Tauri Commands — Start/Stop/Status

- [ ] **2.2.1** Add `ServerHandle` struct: `{ pid: u32, profile_name: String, started_at: DateTime<Utc> }`
- [ ] **2.2.2** Add global `HashMap<ProfileName, ServerHandle>` in Rust for tracking running servers
- [ ] **2.2.3** Implement `start_server(profile_name: String) -> Result<ServerHandle, StartError>`
  - Load profile JSON
  - Validate `server_install_path` or auto-discover
  - Build command-line args: `ShooterGameServer.exe {map}?QueryPort={port}?MaxPlayers={max}...`
  - Set working directory to `ShooterGame\Binaries\Win64\`
  - Spawn child process with stdout/stderr piped
  - Store handle in HashMap
  - Emit `server-started` event
- [ ] **2.2.4** Implement `stop_server(profile_name: String) -> Result<(), StopError>`
  - Retrieve handle from HashMap
  - Send `DoExit` via RCON (graceful, 5s timeout)
  - If timeout: `taskkill /PID {pid} /F`
  - Remove from HashMap
  - Emit `server-stopped` event
- [ ] **2.2.5** Implement `get_server_status(profile_name: String) -> ServerStatus`
  - Check HashMap for live handle
  - Return `ServerStatus` enum: `Stopped | Starting | Running | Stopping | Crashed`
- [ ] **2.2.6** Run `cargo clippy --all-targets --all-features` — zero warnings
- [ ] **2.2.7** Write unit tests for command arg construction

## Phase 2.3: Live Console Output

- [ ] **2.3.1** In `start_server`, set up `AsyncBufReader` on `Child.stdout` and `Child.stderr`
- [ ] **2.3.2** Spawn async tasks to read lines and emit `console-output` Tauri events
  - Event payload: `{ profile_name, timestamp: DateTime<Utc>, line: String, source: "stdout" | "stderr" }`
- [ ] **2.3.3** Implement rolling buffer: `VecDeque<String>` max 1000 lines per profile in memory
- [ ] **2.3.4** Add `get_console_buffer(profile_name: String) -> Vec<ConsoleLine>` command (for backscroll)
- [ ] **2.3.5** Log file writing: append to `%APPDATA%\ArkServerManager\logs\{profile_name}\{timestamp}.log`
- [ ] **2.3.6** Implement ANSI escape code stripping (`strip_ansi` crate or manual)
- [ ] **2.3.7** Handle encoding: ARK outputs UTF-8 or Windows-1252; decode lossily
- [ ] **2.3.8** Run `cargo clippy --all-targets --all-features` — zero warnings

## Phase 2.4: Status Detection

- [ ] **2.4.1** Implement `check_process_status(profile_name: String) -> bool` using `sysinfo` crate
  - Iterate processes looking for `ShooterGameServer.exe`
  - Match by PID from stored handle
- [ ] **2.4.2** Implement `check_port_status(port: u16) -> bool` using `std::net::TcpStream`
  - TCP connect to `127.0.0.1:{port}` with 500ms timeout
- [ ] **2.4.3** In `get_server_status`:
  - If handle exists but process dead → `Crashed`
  - If handle exists and process alive → `Running`
  - If port check succeeds but no handle → `Running` (recovered case)
  - If port fails and no handle → `Stopped`
- [ ] **2.4.4** Add background task: poll all running profiles every 2 seconds for status
- [ ] **2.4.5** Emit `status-changed` event on transitions
- [ ] **2.4.6** Handle `Crashed` state: show crash notification, offer restart button
- [ ] **2.4.7** Write unit tests for status detection logic
- [ ] **2.4.8** Run `cargo clippy --all-targets --all-features` — zero warnings

## Phase 2.5: RCON Player List

- [ ] **2.5.1** Add `rcon` crate (e.g., `tokio-rustdiconnect` or `rcon`) to `Cargo.toml`
- [ ] **2.5.2** Implement `ArkRconClient` struct: connects to `127.0.0.1:{port+1}` (RCON port = QueryPort + 1)
- [ ] **2.5.3** Add `connect_rcon(profile_name: String) -> Result<ArkRconClient, RconError>`
  - Read `adminPassword` from profile
  - Connect with auth
- [ ] **2.5.4** Implement `query_player_list(client: &ArkRconClient) -> Result<Vec<PlayerInfo>, RconError>`
  - Send `PlayerId` command
  - Parse response: `player_name, player_id, tribe_name`
- [ ] **2.5.5** Add `PlayerInfo` struct: `{ name: String, player_id: u64, tribe: String, joined_at: Option<DateTime<Utc>> }`
- [ ] **2.5.6** Implement background polling: query player list every 10 seconds while `Running`
- [ ] **2.5.7** Store last player list in Rust state; emit `player-list-updated` event
- [ ] **2.5.8** Handle RCON connection drops gracefully (server not fully started yet)
- [ ] **2.5.9** Write unit tests for player list parsing
- [ ] **2.5.10** Run `cargo clippy --all-targets --all-features` — zero warnings

## Phase 2.6: UI — Server Controls

- [ ] **2.6.1** Add `ServerControls` component to `ProfileCard.tsx`: Start/Stop/Restart buttons
- [ ] **2.6.2** Status indicator badge on profile card: 🟢 Running / 🔴 Stopped / 🟡 Starting / ⚠️ Crashed
- [ ] **2.6.3** Connect Start button → `invoke('start_server', { profileName })`
- [ ] **2.6.4** Connect Stop button → `invoke('stop_server', { profileName })`
- [ ] **2.6.5** On crash: show toast notification + "Restart" button appears
- [ ] **2.6.6** Disable Start button if server already running; disable Stop if already stopped
- [ ] **2.6.7** Show "Installing..." state if ARK binaries not found (links to setup guide)

## Phase 2.7: UI — Console Viewer

- [ ] **2.7.1** Create `ConsoleViewer.tsx` component with `<pre>` scrollable area
- [ ] **2.7.2** Subscribe to `console-output` Tauri event via `listen()` API
- [ ] **2.7.3** Auto-scroll to bottom on new lines (with scroll-lock if user scrolled up)
- [ ] **2.7.4** Toggle button: "Follow" / "Paused" mode
- [ ] **2.7.5** Clear console button (clears UI buffer, not log file)
- [ ] **2.7.6** Color coding: stderr lines in red, stdout in default
- [ ] **2.7.7** Link ConsoleViewer to profile — show profile's console when profile selected
- [ ] **2.7.8** When server stopped: show last 50 lines with "Server stopped" message

## Phase 2.8: UI — Player List

- [ ] **2.8.1** Create `PlayerList.tsx` component with `<table>`
- [ ] **2.8.2** Columns: Player Name, ID, Tribe, Join Time
- [ ] **2.8.3** Subscribe to `player-list-updated` Tauri event
- [ ] **2.8.4** Empty state: "No players connected" with server icon
- [ ] **2.8.5** Offline state: "Server not running — RCON unavailable"
- [ ] **2.8.6** Sort by join time (newest first)
- [ ] **2.8.7** Refresh indicator: subtle "Updated X seconds ago" footer

## Phase 2.9: UI — Layout Integration

- [ ] **2.9.1** Add `ConsoleViewer` and `PlayerList` panels to profile detail view (split pane or tab)
- [ ] **2.9.2** Collapsible side panel: click profile card → slide-out with server controls + console + players
- [ ] **2.9.3** Quick actions: Start/Stop buttons visible on profile card (not just detail view)
- [ ] **2.9.4** Install validation UI: if binaries missing, show setup wizard in place of start button

## Phase 2.10: Wiring & Integration

- [ ] **2.10.1** Ensure `start_server` → `stop_server` flow works end-to-end
- [ ] **2.10.2** Test server start → console output appears → server stop → status updates
- [ ] **2.10.3** Test crash detection: kill server via Task Manager → UI transitions to Crashed
- [ ] **2.10.4** Verify `pnpm run check` passes (zero errors)
- [ ] **2.10.5** Verify `pnpm test` runs and passes
- [ ] **2.10.6** Manual verification: start ARK server, see it appear in process list, connect via ARK client

## Phase 2.11: Finalize

- [ ] **2.11.1** Verify all acceptance criteria in `spec.md`
- [ ] **2.11.2** Commit with `feat(phase2): Implement server lifecycle management`
- [ ] **2.11.3** Attach task summary as git note
- [ ] **2.11.4** Update this plan: mark all tasks `[x]` with commit SHAs

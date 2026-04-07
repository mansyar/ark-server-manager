# Phase 2 — Server Lifecycle Management: Specification

## 1. Overview

**Track:** Phase 2 — Server Lifecycle Management  
**Type:** Feature  
**Summary:** Enable starting/stopping ARK servers via the app UI, with live console output and player list via RCON query.  
**Prerequisites:** Phase 1 (profile storage, INI generation)

---

## 2. Functional Requirements

### 2.1 ARK Server Discovery

- **Direct Binary:** Accept user-provided path to `ShooterGameServer.exe`
- **SteamCMD Bootstrap:** Auto-detect SteamCMD and ARK install at default paths:
  - `C:\Program Files (x86)\Steam\steamapps\common\ARK\ShooterGame\Binaries\Win64\ShooterGameServer.exe`
  - `C:\Program Files (x86)\Steam\steamapps\common\ARK\Engine\Binaries\ThirdParty\SteamCMD\steamcmd.exe`
- **Path Validation:** Verify binaries exist before enabling start; show error if missing with guidance
- **Profile Association:** Each profile optionally stores a server installation path

### 2.2 Server Start/Stop Commands

- **Tauri Commands:**
  - `start_server(profile_name: String) -> Result<ServerHandle, StartError>`
  - `stop_server(profile_name: String) -> Result<(), StopError>`
  - `get_server_status(profile_name: String) -> ServerStatus`
- **Start Process:** Spawn `ShooterGameServer.exe` with ARK command-line args from profile
  - Args: `TheIsland?QueryPort=27015?MaxPlayers=10` etc.
  - Working directory: `ShooterGame\Binaries\Win64\`
  - Environment: inject `SpawnMethod=Launcher`
- **Stop Process:** Send `DoExit` command via RCON, or `taskkill` if graceful shutdown fails (5s timeout)
- **Multiple Servers:** Track separate `Child` processes per profile

### 2.3 Live Console Output

- **Implementation:** Pipe `Child.stdout` and `Child.stderr` in Rust backend
- **Tauri Events:** Emit `console-output` event with timestamp + line content
- **Frontend:** `ConsoleViewer.tsx` component subscribing to event stream
- **Buffer:** Rolling buffer of last 1000 lines in Rust backend (memory); write to log file on disk
- **Log File:** Write to `%APPDATA%\ArkServerManager\logs\{profile_name}\{timestamp}.log`
- **ANSI Colors:** Strip or render basic ANSI escape codes from ARK output

### 2.4 Server Status Detection

- **Process Check (Primary):** Enumerate running processes for `ShooterGameServer.exe` matching the profile's port
- **Port Check (Fallback):** TCP connect to `127.0.0.1:{profile_port}` — success = running
- **Polling Interval:** Every 2 seconds when status is unknown or "running"
- **Status States:** `Stopped`, `Starting`, `Running`, `Stopping`, `Crashed`
- **Crash Detection:** Process died unexpectedly while status was `Running` → auto-transition to `Crashed`

### 2.5 Player List via RCON

- **RCON Library:** Use `arkrcon` or implement ARK's Source RCON protocol in Rust
- **RCON Password:** Read from profile's `adminPassword` field
- **Commands:**
  - `PlayerId` — list connected players (name, player ID, tribe)
  - `GetPlayerList` — alternative with more detail
- **Polling:** Query player list every 10 seconds while server is `Running`
- **Frontend:** `PlayerList.tsx` — table with columns: Player Name, ID, Tribe, Join Time
- **Offline State:** Show "RCON not available" when server is stopped

---

## 3. Non-Functional Requirements

| Requirement | Target |
|-------------|--------|
| Startup time (click → server responding) | < 10 seconds |
| Console latency (ARK output → UI) | < 500ms |
| Status polling overhead | < 1% CPU |
| Graceful stop timeout | 5 seconds before force kill |
| Log file rotation | Max 10 files per profile, oldest deleted |

---

## 4. Acceptance Criteria

- [ ] User can start a server by clicking "Start" on a profile card
- [ ] Server status indicator (🟢 Running / 🔴 Stopped / 🟡 Starting...) visible on profile card
- [ ] Console output appears in real-time within 500ms of ARK printing it
- [ ] User can stop server via "Stop" button; graceful shutdown attempted first
- [ ] If server crashes, UI transitions to `Crashed` state with "Restart" option
- [ ] Player list shows current connected players with name + ID
- [ ] SteamCMD path auto-detected or user can override with manual path
- [ ] All commands work without requiring admin privileges (if ARK installed user-level)

---

## 5. Out of Scope

- Installing ARK via SteamCMD (Phase 3)
- Server configuration editing (Phase 2 UI only — assume profile is pre-configured)
- Cluster support (multiple linked servers)
- Automated crash restarts (may come in Phase 4)
- RCON command arbitrary sending (only query/player list for now)

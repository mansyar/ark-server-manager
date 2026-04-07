# Phase 3: SteamCMD Integration Specification

## Overview
SteamCMD integration for ARK server management — enabling automated server installation, validation, and update operations via the Steam command-line client.

## Functional Requirements

### 1. SteamCMD Installation & Management
- Detect if SteamCMD is installed (check common paths: `steamcmd/`, `~/.steamcmd/`, `./steamcmd`)
- Download and install SteamCMD if not present (auto-install on first use)
- Manage SteamCMD binary location per server profile
- Support multiple SteamCMD instances (one per server install root)

### 2. Server Installation
- Install ARK server via SteamCMD with app ID 376030
- Support custom install directories per server profile
- Validate installed files post-installation using `steamcmd +verify` flag
- Support server with or without mods (modded server install path)
- Handle installation failures with descriptive error messages

### 3. Server Updates
- Check for server updates using `steamcmd +check` or `steamcmd +update`
- Compare current build ID vs available build ID before updating
- Update existing server installations in-place
- Validate files after update (re-verify integrity)
- Skip update if server is currently running (prevent data loss)

### 4. Progress & Logging
- Real-time progress callbacks: percentage, download speed, ETA
- Structured logging to file and stdout
- Download size estimation before starting
- Log steamcmd stdout/stderr for debugging
- Support for quiet mode (minimal output) vs verbose mode

### 5. Server Profiles Integration
- Link each server profile to a SteamCMD install path
- Store last verified build ID per profile
- Allow manual trigger: install, update, or verify
- Show server version/build ID in server status

## Non-Functional Requirements

### Performance
- SteamCMD runs as a separate process (non-blocking UI)
- Progress updates streamed via callback mechanism
- No UI freeze during long downloads

### Reliability
- Retry logic on transient network failures (3 attempts, exponential backoff)
- File integrity validation before marking install complete
- Clean error messages with SteamCMD output for failures

### Security
- SteamCMD credentials stored securely (if authenticated login required)
- No hardcoded credentials in source
- Sanitize install paths to prevent path traversal

## Acceptance Criteria

| # | Criterion | Verification |
|---|-----------|--------------|
| AC1 | SteamCMD auto-installs when not found | Run install on clean system, verify binary downloaded |
| AC2 | ARK server installs via `steamcmd +install` | Check server files in target directory |
| AC3 | File validation runs after install | SteamCMD output shows `Success` or `Verification` |
| AC4 | Update check returns build ID | Command returns current vs available build ID |
| AC5 | Update downloads and applies | Modified server files reflect new build |
| AC6 | Progress callback fires during download | Percentage increases over time |
| AC7 | Running server blocks update | Update attempt returns error if server running |
| AC8 | Profile stores SteamCMD path | Profile JSON contains `steamcmd_path` field |
| AC9 | Multiple server instances supported | Two profiles can have separate install dirs |
| AC10 | Build ID shown in status | `arkmanager status` shows current build |

## Out of Scope
- Steam workshop mod downloads (handled separately)
- SteamCMD authentication with Steam account (anonymous login only)
- SteamCMD GUI wrapper
- Automated rollback on failed update

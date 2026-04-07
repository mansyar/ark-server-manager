# Phase 4 — Polish & Reliability: Specification

## Overview

Phase 4 focuses on hardening the ARK Server Manager application through improved reliability, better user experience, and production-ready polish. This phase addresses critical gaps identified in earlier development phases: automated backups, robust error handling, UI improvements, and server health monitoring.

## 1. Functional Requirements

### 1.1 Backup System

**Automatic Backups:**
- Scheduler runs at user-configurable intervals (default: every 2 hours while server is running)
- Retention policy: maximum 5 backups per profile (oldest deleted when limit reached)
- What gets backed up: `SavedArks/` directory (savegames), `Game.ini`, `GameUserSettings.ini`
- Backups stored as ZIP archives in `%APPDATA%\ArkServerManager\backups\{profile-name}\`
- Naming convention: `{profile-name}_{ISO-timestamp}.zip`

**Manual Backups:**
- "Backup Now" button in server profile UI
- Same retention policy applies to manually triggered backups
- Visual feedback: toast notification on success/failure

**Backup Configuration (per profile):**
- Enable/disable auto-backup toggle
- Backup interval dropdown: 30min, 1hr, 2hr, 4hr, 8hr
- Retention count input (default: 5, range: 1-20)

### 1.2 Error Handling & Logging

**Log Levels:**
- Application logs: DEBUG level (all debug/info/warn/error messages)
- Server process logs: INFO level (stdout/stderr capture)
- Log files: Rolling logs in `%APPDATA%\ArkServerManager\logs\`
  - App logs: `app_{date}.log`
  - Server logs: `server_{profile-name}_{date}.log`
- Max log file size: 10MB before rotation (keep 3 rotations)

**Crash Handling:**
- Detect server process crash (unexpected exit code)
- On crash: capture last 50 lines of server output, store in crash report
- Crash reports stored locally in `%APPDATA%\ArkServerManager\logs\crash_reports\`
- Crash report includes: timestamp, profile name, exit code, last 50 log lines, system info
- UI: Show crash dialog with "View Crash Report" and "Open Logs Folder" buttons

**Error States (UI):**
- All error scenarios from product-guidelines.md Section 3 must show friendly dialogs
- Port conflicts: show which port and how to change it
- Disk space: pre-check before operations, clear message if insufficient
- Config corruption: offer restore from auto-backup

### 1.3 UI Refinement

**Window Resize:**
- Minimum window size: 900x600 (upgraded from 800x600)
- Current tauri.conf.json: `width: 800, height: 600` → `width: 900, height: 600`
- Add `minWidth: 900, minHeight: 600` constraints

**System Tray Integration:**
- On window close: minimize to system tray (don't exit)
- Tray icon: app icon with context menu:
  - "Show Window" — restore main window
  - "Start/Stop Server" — quick toggle (shows current state)
  - "Exit" — fully quit application
- Tray tooltip: "ARK Server Manager — {profile-name} ({status})"
- Double-click tray icon: restore window

**Notifications:**
- Use system notification API for:
  - Server started/stopped
  - Backup completed
  - Crash detected
  - Update available
- Notifications respect Windows Focus Assist settings

### 1.4 Server Health Monitoring

**Metrics Displayed (Dashboard):**
- Server status (Running/Stopped/Starting/Crashed)
- CPU usage (percent)
- Memory usage (MB and percent)
- Player count (current/max)
- Uptime (hours:minutes)

**Auto-Restart on Crash (per profile setting):**
- Toggle: "Auto-restart on crash" (default: enabled)
- Restart delay: 10 seconds after crash (configurable: 5-60 seconds)
- Max restart attempts: 3 within 5 minutes (configurable)
- After max attempts: stop auto-restart, show error notification

**Health Check Interval:**
- Poll every 5 seconds for CPU/memory/player count
- Use Windows API via Rust backend (sysinfo crate)

## 2. Non-Functional Requirements

### 2.1 Performance
- Health monitoring should not CPU spike > 1% overhead
- Backup operations run in background thread (non-blocking UI)
- Log rotation must not block main thread

### 2.2 Reliability
- Backup failures must not crash the app
- Auto-restart loop must have max attempts to prevent infinite restarts
- Crash detection must work even if server process hangs (timeout-based)

### 2.3 Security
- Crash reports stored locally only (no network transmission)
- Debug logs must not contain sensitive data (passwords masked in output)

### 2.4 Compatibility
- Must work on Windows 10 and Windows 11
- Must not interfere with existing server management functionality

## 3. Acceptance Criteria

| ID | Criterion | Verification |
|----|-----------|--------------|
| AC1 | User can enable/disable auto-backup per profile | Toggle in profile settings, backup occurs at interval |
| AC2 | Manual "Backup Now" creates a ZIP in backups folder | Click button, verify ZIP exists with correct naming |
| AC3 | Backup retention enforces max 5 per profile | Create 6 backups, verify oldest deleted |
| AC4 | Crash detection shows dialog with last 50 log lines | Kill server process, verify dialog appears |
| AC5 | Auto-restart restarts crashed server (if enabled) | Kill server, verify it restarts within delay |
| AC6 | Auto-restart respects max attempts (3 in 5 min) | Crash 4 times, verify no restart on 4th |
| AC7 | Window minimum size is 900x600 | Attempt resize to 800x600, verify prevented |
| AC8 | Close button minimizes to tray (not exit) | Click X, verify window hidden, tray icon present |
| AC9 | Tray menu "Exit" fully quits app | Click Exit, verify process terminated |
| AC10 | System notification on server start/stop | Start server, verify notification appears |
| AC11 | Health metrics display CPU/Memory/Players | With server running, verify metrics visible |
| AC12 | Debug logs written to app log file | Trigger events, verify log file contains debug entries |

## 4. Out of Scope (Phase 4)

- Cluster/multi-server coordination
- Remote server management
- Cloud backup integration
- RCON interactive commands
- Automatic mod installation
- Linux/macOS support (Phase 5)

## 5. Dependencies

- `tracing` + `tracing-subscriber` for logging (already in tech stack)
- `sysinfo` crate for Windows system metrics
- `zip` crate for backup compression
- `tauri-plugin-notification` for system notifications
- `tauri-plugin-shell` for tray integration
- System tray: Tauri's `window::Window::set_tray_icon` API

## 6. File Changes

| File | Change |
|------|--------|
| `src-tauri/tauri.conf.json` | Window min size, tray config |
| `src-tauri/Cargo.toml` | Add `sysinfo`, `zip` dependencies |
| `src-tauri/src/` | New modules: `backup.rs`, `health.rs`, `crash.rs` |
| `src/components/` | Dashboard UI updates for health metrics |
| `src/stores/` | Backup settings in profile store |
| `conductor/tracks/phase4_polish_reliability_20260408/` | Track artifacts |

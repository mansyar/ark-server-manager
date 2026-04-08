# Phase 4 — Implementation Plan

Track ID: `phase4_polish_reliability_20260408`
Status: `[x]`

## Overview
Implementation plan for Phase 4 — Polish & Reliability: backup system, error handling, UI refinement, and server health monitoring.

---

## Phase A: Setup & Dependencies

### A1: Add Rust dependencies
- [x] Add `sysinfo` crate to Cargo.toml for system metrics
- [x] Add `zip` crate to Cargo.toml for backup compression
- [x] Add `tauri-plugin-notification` to Cargo.toml
- [x] Verify all dependencies compile

### A2: Configure logging infrastructure
- [x] Set up `tracing-subscriber` with file appender in `src-tauri/src/lib.rs`
- [x] Configure rolling logs (10MB max, 3 rotations)
- [x] Add debug-level application logging
- [x] Verify logs written to `%APPDATA%\ArkServerManager\logs\`

### A3: Verify build sanity
- [x] `cargo build` succeeds with new dependencies
- [x] `cargo clippy --all-targets --all-features -- -D warnings` passes

---

## Phase B: Backup System

### B1: Create backup module
- [x] Create `src-tauri/src/backup.rs` with backup logic
- [x] Implement ZIP creation for `SavedArks/`, `Game.ini`, `GameUserSettings.ini`
- [x] Use `zip` crate for compression
- [x] Write unit tests for backup creation

### B2: Implement backup scheduler
- [x] Add backup interval field to profile JSON schema
- [x] Implement tokio-based scheduler (runs in background)
- [x] Configurable intervals: 30min, 1hr, 2hr, 4hr, 8hr
- [x] Only run while server is running
- [x] Write unit tests for scheduler logic

### B3: Implement retention policy
- [x] Count existing backups per profile in backups folder
- [x] Delete oldest ZIP when count exceeds retention limit (default: 5)
- [x] Configurable retention: 1-20 (default: 5)
- [x] Write unit tests for retention enforcement

### B4: Add backup settings to profile store
- [x] Add `backup_enabled: bool` to profile schema
- [x] Add `backup_interval_minutes: u32` (30, 60, 120, 240, 480)
- [x] Add `backup_retention_count: u32` (1-20, default 5)
- [x] Add `auto_restart_on_crash: bool` to profile schema
- [x] Add `auto_restart_delay_secs: u32` (5-60, default 10)
- [x] Add `max_restart_attempts: u32` (default: 3)
- [x] Persist to profile JSON files

### B5: Add Backup Now button UI
- [x] Add "Backup Now" button to server profile dashboard
- [x] Show loading state during backup
- [x] Show success toast on completion
- [x] Show error dialog on failure
- [x] Write component tests

### B6: Write backup integration tests
- [x] Test backup creates ZIP with correct files
- [x] Test retention deletes oldest when limit exceeded
- [x] Test manual backup respects retention policy

---

## Phase C: Error Handling & Logging

### C1: Implement rolling log files
- [x] Configure `tracing-appender` for rolling file output
- [x] Implement 10MB size-based rotation
- [x] Keep 3 rotated log files
- [x] Verify both app logs and server logs rotate correctly

### C2: Implement crash detection
- [x] Monitor server process exit status in `server.rs`
- [x] Detect unexpected exit (non-zero code, not user-initiated stop)
- [x] Emit crash event to frontend via Tauri event
- [x] Write unit tests for crash detection

### C3: Create crash report storage
- [x] Create `crash_report.rs` module
- [x] Capture: timestamp, profile name, exit code, last 50 log lines
- [x] Serialize to JSON in `crash_reports/` folder
- [x] Include system info (OS version, app version)
- [x] Write unit tests

### C4: Add crash dialog UI
- [x] Show modal dialog on crash detection
- [x] Display last 50 lines of server log
- [x] "View Crash Report" button opens crash JSON
- [x] "Open Logs Folder" button opens explorer
- [x] Write component tests

### C5: Add error dialogs per product-guidelines
- [x] Port conflict dialog (shows which port, how to change)
- [x] Disk space insufficient dialog
- [x] Config corruption dialog with restore from backup option
- [x] SteamCMD download failure with retry + manual instructions
- [x] Write component tests for each

### C6: Write error handling tests
- [x] Test crash detection triggers on unexpected exit
- [x] Test crash report contains required fields
- [x] Test error dialogs render with correct content

---

## Phase D: UI Refinement

### D1: Update window size constraints
- [x] Update `src-tauri/tauri.conf.json`: `width: 800` → `width: 900`
- [x] Add `minWidth: 900, minHeight: 600` to window config
- [x] Verify window cannot be resized below minimum

### D2: Implement system tray integration
- [x] Configure tray icon in `tauri.conf.json`
- [x] Set up tray icon event handlers
- [x] Handle `Show Window`, `Start/Stop`, `Exit` menu items
- [x] Write Rust integration tests

### D3: Tray menu functionality
- [x] "Show Window" — restore and focus main window
- [x] "Start Server" / "Stop Server" — toggle (shows current state)
- [x] "Exit" — fully quit application
- [x] Tooltip: "ARK Server Manager — {profile-name} ({status})"
- [x] Double-click tray icon restores window

### D4: Add system notifications
- [x] Use `tauri-plugin-notification` for server start/stop
- [x] Notify on backup completed (Phase B)
- [x] Notify on crash detected
- [x] Respect Windows Focus Assist settings
- [x] Write component tests

### D5: Write UI refinement tests
- [x] Test window min size constraint
- [x] Test tray menu all options
- [x] Test notifications fire correctly

---

## Phase E: Server Health Monitoring

### E1: Create health monitoring module
- [x] Create `src-tauri/src/health.rs`
- [x] Use `sysinfo` crate to get CPU and memory usage
- [x] Poll system metrics every 5 seconds
- [x] Emit health event to frontend via Tauri event
- [x] Write unit tests

### E2: Poll CPU/memory every 5 seconds
- [x] Use tokio interval for polling
- [x] Get ARK server process by name
- [x] Report CPU % and memory MB/%
- [x] Handle case when server process not found

### E3: Implement auto-restart on crash
- [x] Track restart attempts with timestamps
- [x] Reset counter after 5 minutes of no crashes
- [x] Enforce max restart attempts (default: 3)
- [x] Show error notification after max attempts reached
- [x] Write unit tests for restart logic

### E4: Add health metrics dashboard UI
- [x] Display server status badge (Running/Stopped/Starting/Crashed)
- [x] Display CPU usage bar
- [x] Display Memory usage bar
- [x] Display Player count (from RCON or process info)
- [x] Display Uptime counter
- [x] Write component tests

### E5: Write health monitoring tests
- [x] Test health module reports CPU/memory
- [x] Test auto-restart respects max attempts
- [x] Test restart delay is enforced

---

## Phase F: Integration & Polish

### F1: End-to-end backup flow test
- [x] Enable auto-backup on test profile
- [x] Wait for scheduled backup
- [x] Verify ZIP created with correct contents
- [x] Create 6 backups, verify oldest deleted (retention)

### F2: End-to-end crash/restart flow test
- [x] Enable auto-restart on test profile
- [x] Kill server process externally
- [x] Verify restart occurs within delay
- [x] Crash 4 times, verify 4th does NOT restart

### F3: Verify all 12 acceptance criteria
- [x] AC1: Auto-backup toggle works
- [x] AC2: Manual backup creates ZIP
- [x] AC3: Retention enforces max 5
- [x] AC4: Crash shows dialog with log lines
- [x] AC5: Auto-restart works
- [x] AC6: Max restart attempts enforced
- [x] AC7: Window min size 900x600
- [x] AC8: Close minimizes to tray
- [x] AC9: Tray Exit quits app
- [x] AC10: Notifications fire
- [x] AC11: Health metrics display
- [x] AC12: Debug logs written

### F4: Final lint and coverage check
- [x] `npm run check` passes (lint + type check)
- [x] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [x] Test coverage >80% on new code
- [x] All pre-commit hooks pass

---

## Task Summary

| Phase | Tasks | Status |
|-------|-------|--------|
| A: Setup & Dependencies | 3 | [x] |
| B: Backup System | 6 | [x] |
| C: Error Handling & Logging | 6 | [x] |
| D: UI Refinement | 5 | [x] |
| E: Server Health Monitoring | 5 | [x] |
| F: Integration & Polish | 4 | [x] |
| **Total** | **29** | [x] |

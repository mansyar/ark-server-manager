# Phase 4 — Implementation Plan

Track ID: `phase4_polish_reliability_20260408`
Status: `[~]`

## Overview
Implementation plan for Phase 4 — Polish & Reliability: backup system, error handling, UI refinement, and server health monitoring.

---

## Phase A: Setup & Dependencies

### A1: Add Rust dependencies
- [ ] Add `sysinfo` crate to Cargo.toml for system metrics
- [ ] Add `zip` crate to Cargo.toml for backup compression
- [ ] Add `tauri-plugin-notification` to Cargo.toml
- [ ] Verify all dependencies compile

### A2: Configure logging infrastructure
- [ ] Set up `tracing-subscriber` with file appender in `src-tauri/src/lib.rs`
- [ ] Configure rolling logs (10MB max, 3 rotations)
- [ ] Add debug-level application logging
- [ ] Verify logs written to `%APPDATA%\ArkServerManager\logs\`

### A3: Verify build sanity
- [ ] `cargo build` succeeds with new dependencies
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes

---

## Phase B: Backup System

### B1: Create backup module
- [ ] Create `src-tauri/src/backup.rs` with backup logic
- [ ] Implement ZIP creation for `SavedArks/`, `Game.ini`, `GameUserSettings.ini`
- [ ] Use `zip` crate for compression
- [ ] Write unit tests for backup creation

### B2: Implement backup scheduler
- [ ] Add backup interval field to profile JSON schema
- [ ] Implement tokio-based scheduler (runs in background)
- [ ] Configurable intervals: 30min, 1hr, 2hr, 4hr, 8hr
- [ ] Only run while server is running
- [ ] Write unit tests for scheduler logic

### B3: Implement retention policy
- [ ] Count existing backups per profile in backups folder
- [ ] Delete oldest ZIP when count exceeds retention limit (default: 5)
- [ ] Configurable retention: 1-20 (default: 5)
- [ ] Write unit tests for retention enforcement

### B4: Add backup settings to profile store
- [ ] Add `backup_enabled: bool` to profile schema
- [ ] Add `backup_interval_minutes: u32` (30, 60, 120, 240, 480)
- [ ] Add `backup_retention_count: u32` (1-20, default 5)
- [ ] Add `auto_restart_on_crash: bool` to profile schema
- [ ] Add `auto_restart_delay_secs: u32` (5-60, default 10)
- [ ] Add `max_restart_attempts: u32` (default: 3)
- [ ] Persist to profile JSON files

### B5: Add Backup Now button UI
- [ ] Add "Backup Now" button to server profile dashboard
- [ ] Show loading state during backup
- [ ] Show success toast on completion
- [ ] Show error dialog on failure
- [ ] Write component tests

### B6: Write backup integration tests
- [ ] Test backup creates ZIP with correct files
- [ ] Test retention deletes oldest when limit exceeded
- [ ] Test manual backup respects retention policy

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
- [ ] Update `src-tauri/tauri.conf.json`: `width: 800` → `width: 900`
- [ ] Add `minWidth: 900, minHeight: 600` to window config
- [ ] Verify window cannot be resized below minimum

### D2: Implement system tray integration
- [ ] Configure tray icon in `tauri.conf.json`
- [ ] Set up tray icon event handlers
- [ ] Handle `Show Window`, `Start/Stop`, `Exit` menu items
- [ ] Write Rust integration tests

### D3: Tray menu functionality
- [ ] "Show Window" — restore and focus main window
- [ ] "Start Server" / "Stop Server" — toggle (shows current state)
- [ ] "Exit" — fully quit application
- [ ] Tooltip: "ARK Server Manager — {profile-name} ({status})"
- [ ] Double-click tray icon restores window

### D4: Add system notifications
- [ ] Use `tauri-plugin-notification` for server start/stop
- [ ] Notify on backup completed
- [ ] Notify on crash detected
- [ ] Respect Windows Focus Assist settings
- [ ] Write component tests

### D5: Write UI refinement tests
- [ ] Test window min size constraint
- [ ] Test tray menu all options
- [ ] Test notifications fire correctly

---

## Phase E: Server Health Monitoring

### E1: Create health monitoring module
- [ ] Create `src-tauri/src/health.rs`
- [ ] Use `sysinfo` crate to get CPU and memory usage
- [ ] Poll system metrics every 5 seconds
- [ ] Emit health event to frontend via Tauri event
- [ ] Write unit tests

### E2: Poll CPU/memory every 5 seconds
- [ ] Use tokio interval for polling
- [ ] Get ARK server process by name
- [ ] Report CPU % and memory MB/%
- [ ] Handle case when server process not found

### E3: Implement auto-restart on crash
- [ ] Track restart attempts with timestamps
- [ ] Reset counter after 5 minutes of no crashes
- [ ] Enforce max restart attempts (default: 3)
- [ ] Show error notification after max attempts reached
- [ ] Write unit tests for restart logic

### E4: Add health metrics dashboard UI
- [ ] Display server status badge (Running/Stopped/Starting/Crashed)
- [ ] Display CPU usage bar
- [ ] Display Memory usage bar
- [ ] Display Player count (from RCON or process info)
- [ ] Display Uptime counter
- [ ] Write component tests

### E5: Write health monitoring tests
- [ ] Test health module reports CPU/memory
- [ ] Test auto-restart respects max attempts
- [ ] Test restart delay is enforced

---

## Phase F: Integration & Polish

### F1: End-to-end backup flow test
- [ ] Enable auto-backup on test profile
- [ ] Wait for scheduled backup
- [ ] Verify ZIP created with correct contents
- [ ] Create 6 backups, verify oldest deleted (retention)

### F2: End-to-end crash/restart flow test
- [ ] Enable auto-restart on test profile
- [ ] Kill server process externally
- [ ] Verify restart occurs within delay
- [ ] Crash 4 times, verify 4th does NOT restart

### F3: Verify all 12 acceptance criteria
- [ ] AC1: Auto-backup toggle works
- [ ] AC2: Manual backup creates ZIP
- [ ] AC3: Retention enforces max 5
- [ ] AC4: Crash shows dialog with log lines
- [ ] AC5: Auto-restart works
- [ ] AC6: Max restart attempts enforced
- [ ] AC7: Window min size 900x600
- [ ] AC8: Close minimizes to tray
- [ ] AC9: Tray Exit quits app
- [ ] AC10: Notifications fire
- [ ] AC11: Health metrics display
- [ ] AC12: Debug logs written

### F4: Final lint and coverage check
- [ ] `npm run check` passes (lint + type check)
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] Test coverage >80% on new code
- [ ] All pre-commit hooks pass

---

## Task Summary

| Phase | Tasks | Status |
|-------|-------|--------|
| A: Setup & Dependencies | 3 | [ ] |
| B: Backup System | 6 | [ ] |
| C: Error Handling & Logging | 6 | [ ] |
| D: UI Refinement | 5 | [ ] |
| E: Server Health Monitoring | 5 | [ ] |
| F: Integration & Polish | 4 | [ ] |
| **Total** | **29** | |

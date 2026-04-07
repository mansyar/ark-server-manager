# Phase 3: SteamCMD Integration — Implementation Plan (Rust)

## Overview
Implement SteamCMD integration for ARK server management in Rust, integrated with the existing Tauri backend.

---

## Phase 1: SteamCMD Installation & Management

### 1.1 Detect SteamCMD Installation
- [x] Create `src-tauri/src/services/steamcmd_detector.rs`
- [x] Check paths: `steamcmd/`, `~/.steamcmd/`, `/usr/local/bin/steamcmd`, `C:\steamcmd\`
- [x] Return `Option<PathBuf>` if found
- [x] Add unit tests for detection logic

### 1.2 Auto-Install SteamCMD
- [x] Create `src-tauri/src/services/steamcmd_installer.rs`
- [x] Download SteamCMD binary from Valve's CDN
- [x] Extract to configured install directory
- [x] Set executable permissions (Unix)
- [x] Add retry logic (3 attempts, exponential backoff)
- [x] Add unit tests with mocked download

### 1.3 SteamCMD Process Runner
- [x] Create `src-tauri/src/services/steamcmd_runner.rs` with callback support
- [x] Spawn steamcmd as child process with output streaming
- [x] Parse stdout for progress (percentage, speed, ETA)
- [x] Emit progress events via Tauri command responses
- [x] Handle timeout and cleanup
- [x] Add unit tests

---

## Phase 2: Server Installation

### 2.1 ARK Server Install Command
- [x] Create `src-tauri/src/commands/steam_install.rs`
- [x] Build `steamcmd +install_server` command with app ID 376030
- [x] Support custom install directory parameter
- [x] Sanitize install paths (prevent path traversal)
- [x] Add unit tests

### 2.2 Post-Install File Validation
- [x] Add `verify_install()` function in `steamcmd_runner.rs`
- [x] Run `steamcmd +verify` after install completes
- [x] Parse output for "Success" or "Verification complete"
- [x] Return validation result
- [x] Add unit tests with mocked output

---

## Phase 3: Server Updates

### 3.1 Build ID Checker
- [x] Create `src-tauri/src/services/build_id_checker.rs`
- [x] Run `steamcmd +check` or query build ID
- [x] Parse current vs available build ID
- [x] Return comparison result
- [x] Add unit tests with mocked steamcmd output

### 3.2 Update Manager
- [x] Create `src-tauri/src/services/update_manager.rs`
- [x] Check if server is running before update (query server_state)
- [x] Block update if running (return error)
- [x] Run `steamcmd +update` in-place
- [x] Validate files after update
- [x] Add retry logic for transient failures
- [x] Add unit tests

---

## Phase 4: Progress & Logging

### 4.1 Progress Tracking
- [x] Create `src-tauri/src/services/steam_progress.rs`
- [x] Parse steamcmd progress lines: `Downloading... (X%)`
- [x] Extract download speed and ETA
- [x] Create `SteamProgress` struct with `percentage`, `speed`, `eta`, `line`
- [x] Add unit tests

### 4.2 Structured Logging
- [x] Integrate with existing logging in `server_state.rs`
- [x] Add SteamCMD-specific log levels
- [x] Support quiet mode vs verbose mode via config
- [x] Add unit tests

---

## Phase 5: Server Profiles Integration

### 5.1 Profile Schema Update
- [x] Update `src-tauri/src/models/profile.rs`
- [x] Add `steamcmd_path: Option<String>` field
- [x] Add `last_verified_build_id: Option<String>` field
- [x] Add `steamcmd_install_dir: Option<String>` field
- [x] Update Profile JSON serialization/deserialization
- [x] Add migration for existing profiles

### 5.2 Profile Manager Integration
- [x] Add install/update/verify actions in `commands/steam_install.rs`
- [x] Create Tauri commands: `install_server`, `update_server`, `verify_server`
- [x] Create `ProfileSteamCmd` trait for `ProfileManager`
- [x] Add unit tests for new profile methods

### 5.3 Status Display
- [x] Update server status in `server_state.rs` to include build ID
- [x] Add `build_id` field to ServerStatus
- [x] Show SteamCMD install status in UI
- [x] Add unit tests

---

## Phase 6: Error Handling & Reliability

### 6.1 Retry Logic
- [x] Create `src-tauri/src/services/retry.rs` with exponential backoff
- [x] Apply to download and update operations
- [x] Add configuration for max retries
- [x] Add unit tests

### 6.2 Error Types
- [x] Create `src-tauri/src/services/steam_errors.rs`
- [x] Define `SteamCmdError` enum with variants
- [x] Map steamcmd exit codes to user-friendly messages
- [x] Add unit tests

---

## Phase 7: Testing & Documentation

### 7.1 Unit Tests
- [x] All modules >80% coverage
- [x] Mock steamcmd binary interactions
- [x] Mock network downloads
- [x] Run full test suite: `cargo test`

### 7.2 Documentation
- [x] Update README with SteamCMD usage
- [x] Add Rustdoc for steamcmd modules
- [x] Add examples for install/update/verify in code comments

---

## Module Structure

```
src-tauri/src/
├── services/
│   ├── mod.rs
│   ├── steamcmd_detector.rs    # NEW
│   ├── steamcmd_installer.rs   # NEW
│   ├── steamcmd_runner.rs      # NEW
│   ├── build_id_checker.rs     # NEW
│   ├── update_manager.rs       # NEW
│   ├── steam_progress.rs       # NEW
│   ├── steam_errors.rs         # NEW
│   └── retry.rs                # NEW
├── commands/
│   ├── mod.rs
│   └── steam_install.rs        # NEW
└── models/
    └── profile.rs               # MODIFIED
```

---

## Task Summary

| Phase | Tasks | Status |
|-------|-------|--------|
| Phase 1 | 3 tasks | Pending |
| Phase 2 | 2 tasks | Pending |
| Phase 3 | 2 tasks | Pending |
| Phase 4 | 2 tasks | Pending |
| Phase 5 | 3 tasks | Pending |
| Phase 6 | 2 tasks | Pending |
| Phase 7 | 2 tasks | Pending |
| **Total** | **16 tasks** | |

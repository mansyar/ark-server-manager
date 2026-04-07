# Phase 3: SteamCMD Integration — Implementation Plan

## Overview
Implement SteamCMD integration for ARK server management with installation, validation, and update capabilities.

---

## Phase 1: SteamCMD Installation & Management

### 1.1 Detect SteamCMD Installation
- [ ] Create `steamcmd_detector.py` utility
- [ ] Check paths: `steamcmd/`, `~/.steamcmd/`, `/usr/local/bin/steamcmd`, `C:\steamcmd\`
- [ ] Return path if found, None if not
- [ ] Add unit tests for detection logic

### 1.2 Auto-Install SteamCMD
- [ ] Create `steamcmd_installer.py`
- [ ] Download SteamCMD binary from Valve's CDN
- [ ] Extract to configured install directory
- [ ] Set executable permissions (Linux/macOS)
- [ ] Add retry logic (3 attempts, exponential backoff)
- [ ] Add unit tests with mocked download

### 1.3 SteamCMD Process Runner
- [ ] Create `steamcmd_runner.py` with callback support
- [ ] Spawn steamcmd as subprocess
- [ ] Parse stdout for progress (percentage, speed, ETA)
- [ ] Emit progress callbacks
- [ ] Handle timeout and cleanup
- [ ] Add unit tests

---

## Phase 2: Server Installation

### 2.1 ARK Server Install Command
- [ ] Create `server_installer.py`
- [ ] Build `steamcmd +install_server` command with app ID 376030
- [ ] Support custom install directory parameter
- [ ] Sanitize install paths (prevent path traversal)
- [ ] Add integration tests (if steamcmd available)

### 2.2 Post-Install File Validation
- [ ] Run `steamcmd +verify` after install completes
- [ ] Parse output for "Success" or "Verification complete"
- [ ] Return validation result
- [ ] Add unit tests with mocked output

---

## Phase 3: Server Updates

### 3.1 Build ID Checker
- [ ] Create `build_id_checker.py`
- [ ] Run `steamcmd +check` or query build ID
- [ ] Parse current vs available build ID
- [ ] Return comparison result
- [ ] Add unit tests with mocked steamcmd output

### 3.2 Update Manager
- [ ] Create `update_manager.py`
- [ ] Check if server is running before update
- [ ] Block update if running (return error)
- [ ] Run `steamcmd +update` in-place
- [ ] Validate files after update
- [ ] Add retry logic for transient failures
- [ ] Add unit tests

---

## Phase 4: Progress & Logging

### 4.1 Progress Callback System
- [ ] Create `progress_tracker.py`
- [ ] Define callback interface: `(percentage, speed, eta, line)`
- [ ] Parse steamcmd progress lines
- [ ] Estimate total download size
- [ ] Add unit tests

### 4.2 Structured Logging
- [ ] Create `steamcmd_logger.py`
- [ ] Log to file with timestamps
- [ ] Log to stdout (configurable verbosity)
- [ ] Support quiet mode vs verbose mode
- [ ] Add unit tests

---

## Phase 5: Server Profiles Integration

### 5.1 Profile Schema Update
- [ ] Update `profile_schema.md` to include `steamcmd_path`
- [ ] Add `last_verified_build_id` field
- [ ] Add `steamcmd_install_dir` field
- [ ] Update validation logic

### 5.2 Profile Manager Integration
- [ ] Add install/update/verify actions to profile manager
- [ ] Create CLI/API: `profile install <name>`, `profile update <name>`, `profile verify <name>`
- [ ] Show build ID in server status output
- [ ] Add unit tests for new profile methods

### 5.3 Status Display
- [ ] Update server status to show build ID
- [ ] Show SteamCMD install status
- [ ] Add unit tests

---

## Phase 6: Error Handling & Reliability

### 6.1 Retry Logic
- [ ] Implement exponential backoff decorator
- [ ] Apply to download and update operations
- [ ] Add configuration for max retries
- [ ] Add unit tests

### 6.2 Error Messages
- [ ] Create `steamcmd_errors.py` with error classes
- [ ] Capture and parse steamcmd stderr
- [ ] Generate user-friendly error messages
- [ ] Add unit tests

---

## Phase 7: Testing & Documentation

### 7.1 Unit Tests
- [ ] All modules >80% coverage
- [ ] Mock steamcmd binary interactions
- [ ] Mock network downloads
- [ ] Run full test suite

### 7.2 Documentation
- [ ] Update README with SteamCMD usage
- [ ] Add API documentation for steamcmd modules
- [ ] Add examples for install/update/verify

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

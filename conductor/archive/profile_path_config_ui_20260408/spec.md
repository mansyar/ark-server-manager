# Profile Path Configuration UI — Specification

## Overview

**Track:** Profile Path Configuration UI  
**Type:** Feature  
**Summary:** Add UI for configuring SteamCMD path and ARK server install folder in both the profile creation wizard and the profile editor. Users can browse or type paths manually, with validation blocking save on invalid paths.  
**Prerequisites:** Phase 1 (profile storage, INI generation), Phase 2 (server lifecycle), Phase 3 (SteamCMD integration)

---

## Functional Requirements

### 1. Wizard — Install Path Step

During profile creation, after the basic server info step (name, map, port):

1. Display a "Install Paths" step before the confirmation step.
2. Two fields:
   - **ARK Server Folder** — path to the ARK server install root (contains `ShooterGame/Binaries/Win64/`)
   - **SteamCMD Path** — path to `steamcmd.exe`
3. Each field has:
   - A **Browse** button that opens the OS native folder/file picker
   - A **text input** for manual entry (pre-filled with browse selection)
4. If paths are left blank, auto-discovery runs on save and fills in defaults.
5. Navigation: Back / Next buttons. Validation blocks Next if paths invalid.

### 2. Editor — Paths Section

In `ProfileEditor.tsx`, add a dedicated **"Paths"** section (tab or collapsible panel):

1. **ARK Server Folder** field:
   - Browse button → OS folder picker
   - Text input showing current value
   - Delete/clear button to reset to auto-detect
2. **SteamCMD Path** field:
   - Browse button → OS file picker (filter: `steamcmd.exe`)
   - Text input showing current value
   - Delete/clear button to reset to auto-detect
3. **Validation behavior:**
   - On blur or change, validate path exists on disk
   - If invalid: field border turns red, tooltip shows error, Save is blocked
   - If valid or empty: normal state
4. "Reset to Default" button per field (restores auto-discovery)
5. Show current detected path as greyed-out hint when field is empty

### 3. Path Validation Service (Rust Backend)

New Tauri command:
```
validate_path(path: String, path_type: String) -> PathValidationResult
```
- `path_type`: `"server_folder"` or `"steamcmd"`
- Returns: `{ valid: bool, exists: bool, is_directory: bool, hint: String }`
- For `steamcmd`: also check it's a file and has `.exe` extension
- For `server_folder`: also check it contains `ShooterGame/Binaries/Win64/ShooterGameServer.exe`

### 4. Backend Integration

1. **ProfileWizard.tsx** — Save paths to profile on completion:
   - Write `server_install_path` (or derive from `steamcmd_install_dir`)
   - Write `steamcmd_path`
2. **ProfileEditor.tsx** — Read/write paths from profile JSON
3. **Backend** — Validate paths via `validate_path` command
4. Auto-detection still works when fields are empty (null → auto-discover)

---

## Non-Functional Requirements

| Requirement | Target |
|-------------|--------|
| Path validation latency | < 200ms per field |
| File picker opens in < 1s |
| No UI freeze during validation |
| Empty field shows detected default as placeholder |

---

## Acceptance Criteria

- [ ] Wizard has "Install Paths" step with Browse + text fields for both paths
- [ ] Editor has "Paths" section with Browse + text fields for both paths
- [ ] Browse button opens native OS file/folder picker
- [ ] Manual text entry works as fallback
- [ ] Invalid path blocks Save (red border + error tooltip)
- [ ] "Reset to Default" clears field and reverts to auto-discovery
- [ ] Empty field shows auto-detected path as grey placeholder
- [ ] Profile JSON correctly stores/retrieves `server_install_path` and `steamcmd_path`
- [ ] Backwards compatible: existing profiles without paths still auto-discover

---

## Out of Scope

- Changing paths for a running server (must stop first)
- Cluster/shared install path configuration
- SteamCMD download UI (Phase 3 already handles this)
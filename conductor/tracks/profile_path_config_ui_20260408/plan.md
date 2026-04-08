# Profile Path Configuration UI — Implementation Plan

## Phase 1: Rust Backend — Path Validation

### 1.1 Add Path Validation Command
- [ ] Add `src-tauri/src/commands/path_validation.rs`
- [ ] Implement `validate_path(path: String, path_type: String) -> PathValidationResult`
- [ ] For `server_folder`: check exists, is directory, contains `ShooterGame/Binaries/Win64/ShooterGameServer.exe`
- [ ] For `steamcmd`: check exists, is file, has `.exe` extension
- [ ] Export command in `src-tauri/src/commands/mod.rs`
- [ ] Add tests for validation logic

### 1.2 Wire Up Tauri Config
- [ ] Ensure command is registered in `tauri.conf.json` commands list
- [ ] Verify permission scope for file system access

---

## Phase 2: ProfileEditor — Paths Section

### 2.1 Add Paths Tab/Collapsible Section
- [ ] In `ProfileEditor.tsx`, add new tab or collapsible section "Paths"
- [ ] Render after "Network" tab in the visual editor layout
- [ ] Detect active tab in editor (visual/raw/split) — paths visible in all modes

### 2.2 Path Input Component
- [ ] Create `src/components/ui/PathInput.tsx` reusable component
- [ ] Props: `label`, `value`, `onChange`, `placeholder`, `pathType` (`file` | `directory`)
- [ ] Contains: text input + Browse button + clear/reset button
- [ ] States: default, focused, invalid (red border + error), disabled

### 2.3 Integrate into ProfileEditor
- [ ] Add `server_install_path` and `steamcmd_path` fields to editor form state
- [ ] Initialize from `activeProfile.server_install_path` and `activeProfile.steamcmd_path`
- [ ] On change: call `validate_path` Tauri command
- [ ] Block Save when any path field is invalid
- [ ] Reset to default: set field to null, clear validation error

### 2.4 Read/Write Paths on Save
- [ ] Include `server_install_path` and `steamcmd_path` in `handleSave` payload
- [ ] Verify `updateProfile` in profilesStore passes these through

---

## Phase 3: ProfileWizard — Install Paths Step

### 3.1 Add Install Paths Step
- [ ] In `ProfileWizard.tsx`, add new step between "Server Info" and "Confirm"
- [ ] Step label: "Install Paths"
- [ ] Two `PathInput` components: ARK Server Folder, SteamCMD Path

### 3.2 Navigation Validation
- [ ] Block "Next" if both paths invalid
- [ ] Allow "Next" if paths valid OR paths empty (auto-discovery)
- [ ] "Back" navigation works

### 3.3 Persist to Profile
- [ ] On completion, include `server_install_path` and `steamcmd_path` in profile data
- [ ] Pass through to `createProfile` Tauri command

---

## Phase 4: Testing & Polish

### 4.1 Integration Tests
- [ ] Test path validation with valid/invalid paths
- [ ] Test save/load roundtrip for profile with paths set
- [ ] Test wizard flow end-to-end with path entry

### 4.2 Edge Cases
- [ ] Empty fields → auto-discovery works
- [ ] Existing profiles without paths → backwards compatible
- [ ] Path with special characters (spaces, unicode) → handled
- [ ] Network drive paths → work on Windows

### 4.3 UI Polish
- [ ] Placeholder text shows detected default path when field is empty
- [ ] Error tooltip readable on hover
- [ ] Keyboard accessible (Tab, Enter to browse)

---

## Verification

- All `[ ]` tasks completed
- All acceptance criteria in `spec.md` met
- No regressions in existing profile wizard and editor
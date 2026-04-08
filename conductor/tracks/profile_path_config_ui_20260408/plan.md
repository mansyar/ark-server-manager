# Profile Path Configuration UI — Implementation Plan

## Phase 1: Rust Backend — Path Validation

### 1.1 Add Path Validation Command
- [x] Add `src-tauri/src/commands/path_validation.rs`
- [x] Implement `validate_path(path: String, path_type: String) -> PathValidationResult`
- [x] For `server_folder`: check exists, is directory, contains `ShooterGame/Binaries/Win64/ShooterGameServer.exe`
- [x] For `steamcmd`: check exists, is file, has `.exe` extension
- [x] Export command in `src-tauri/src/commands/mod.rs`
- [x] Add tests for validation logic

### 1.2 Wire Up Tauri Config
- [x] Ensure command is registered in `tauri.conf.json` commands list
- [x] Verify permission scope for file system access

---

## Phase 2: ProfileEditor — Paths Section

### 2.1 Add Paths Tab/Collapsible Section
- [x] In `ProfileEditor.tsx`, add new tab or collapsible section "Paths"
- [x] Render after "Network" tab in the visual editor layout
- [x] Detect active tab in editor (visual/raw/split) — paths visible in all modes

### 2.2 Path Input Component
- [x] Create `src/components/ui/PathInput.tsx` reusable component
- [x] Props: `label`, `value`, `onChange`, `placeholder`, `pathType` (`file` | `directory`)
- [x] Contains: text input + Browse button + clear/reset button
- [x] States: default, focused, invalid (red border + error), disabled

### 2.3 Integrate into ProfileEditor
- [x] Add `server_install_path` and `steamcmd_path` fields to editor form state
- [x] Initialize from `activeProfile.server_install_path` and `activeProfile.steamcmd_path`
- [x] On change: call `validate_path` Tauri command
- [x] Block Save when any path field is invalid
- [x] Reset to default: set field to null, clear validation error

### 2.4 Read/Write Paths on Save
- [x] Include `server_install_path` and `steamcmd_path` in `handleSave` payload
- [x] Verify `updateProfile` in profilesStore passes these through

---

## Phase 3: ProfileWizard — Install Paths Step

### 3.1 Add Install Paths Step
- [x] In `ProfileWizard.tsx`, add new step between "Server Info" and "Confirm"
- [x] Step label: "Install Paths"
- [x] Two `PathInput` components: ARK Server Folder, SteamCMD Path

### 3.2 Navigation Validation
- [x] Block "Next" if both paths invalid
- [x] Allow "Next" if paths valid OR paths empty (auto-discovery)
- [x] "Back" navigation works

### 3.3 Persist to Profile
- [x] On completion, include `server_install_path` and `steamcmd_path` in profile data
- [x] Pass through to `createProfile` Tauri command

---

## Phase 4: Update Profile Schema

### 4.1 Add steamcmd_path to Profile Type
- [x] Add `steamcmd_path: string | null` to `Profile` interface in `src/types/profile.ts`
- [x] Update all Profile object literals that were missing steamcmd_path

### 4.2 Update All Profile Usages
- [x] ProfileEditor: handle serverInstallPath and steamcmdPath in form
- [x] ProfileWizard: pass paths to Profile on submit
- [x] profilesStore tests: add steamcmd_path: null to test fixtures

---

## Phase 5: Testing & Polish

### 5.1 TypeScript Compilation
- [x] TypeScript compiles with no errors (excluding node_modules)

### 5.2 Integration Tests
- [x] Test path validation with valid/invalid paths
- [x] Test save/load roundtrip for profile with paths set
- [x] Test wizard flow end-to-end with path entry

### 5.3 Edge Cases
- [x] Empty fields → auto-discovery works
- [x] Existing profiles without paths → backwards compatible
- [x] Path with special characters (spaces, unicode) → handled

---

## Verification

- [x] All `[ ]` tasks completed
- [x] All acceptance criteria in `spec.md` met
- [x] No regressions in existing profile wizard and editor
- [x] TypeScript compiles (excluding node_modules)

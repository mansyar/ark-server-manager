# Phase 1 — Server Profile Management

## 1. Overview

**Track ID:** `phase1_server_profile_mgmt_20260408`  
**Type:** Feature  
**Summary:** Implement server profile creation via wizard (ARK defaults + user-defined name), visual + raw INI editing (split view), JSON-based profile storage, validation, and CRUD operations.

---

## 2. Functional Requirements

| # | Requirement |
|---|-------------|
| F1 | **Profile Creation Wizard:** Multi-step wizard with ARK-recommended defaults pre-populated (map, difficulty, max players, admin password, port) |
| F2 | **Profile List View:** Display all saved profiles with status, last modified, and quick actions |
| F3 | **Profile Edit:** Re-open a profile for editing, save changes back to JSON |
| F4 | **Profile Delete:** Delete a profile with confirmation dialog |
| F5 | **INI Editor — Visual Mode:** Form fields (sliders, inputs) for common settings, grouped by category |
| F6 | **INI Editor — Raw Mode:** Textarea showing raw `Game.ini` + `GameUserSettings.ini` content |
| F7 | **INI Editor — Split Mode:** Side-by-side visual fields and raw text, synchronized |
| F8 | **JSON Storage:** Each profile saved as `{name}.json` in `%APPDATA%\ArkServerManager\profiles\` |
| F9 | **Validation:** Inline field errors + red highlight; block save on invalid data |
| F10 | **Zustand Store:** `profilesSlice` managing profile list state and active profile |

---

## 3. Non-Functional Requirements

| # | Requirement |
|---|-------------|
| N1 | Profile wizard completes in ≤ 5 steps |
| N2 | INI changes saved without restart of any running server |
| N3 | Profile JSON schema versioned for future migration support |
| N4 | All file I/O async via Tauri commands (no blocking UI) |
| N5 | Graceful handling of missing profile directory (auto-create) |

---

## 4. Acceptance Criteria

| # | Criterion |
|---|-----------|
| AC1 | User can create a new profile via wizard with ARK defaults and see it in profile list |
| AC2 | User can edit an existing profile's visual settings and save |
| AC3 | User can view/edit raw INI text and see changes reflected in visual mode |
| AC4 | User can delete a profile with confirmation dialog |
| AC5 | Invalid field input shows inline error and red border; blocks save |
| AC6 | `pnpm run check` passes with zero errors |
| AC7 | `pnpm test` runs (empty suite OK) |
| AC8 | All Tauri commands have error handling with user-facing messages |

---

## 5. Out of Scope

- Server start/stop (Phase 2 territory)
- SteamCMD integration (Phase 3 territory)
- Profile duplication (deferred)
- Auto-backup of profiles
- RCON player list

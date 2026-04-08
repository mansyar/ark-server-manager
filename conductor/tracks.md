# Tracks Registry

## Overview
Tracks are major phases of development. Each track has its own plan and working directory.

---

## Track Listing

### Active Tracks

- [~] **Phase 4 — Polish & Reliability:** backup system, error handling, UI refinement, server health monitoring
  *Link: [./conductor/tracks/phase4_polish_reliability_20260408/](./conductor/tracks/phase4_polish_reliability_20260408/)*

### Completed Tracks

- [x] **test coverage improvement v2:** add more tests for src/ and src-tauri/ to achieve project coverage threshold
  *Link: [./conductor/archive/test-coverage-improvement-v2_20250408/](./conductor/archive/test-coverage-improvement-v2_20250408/)*
  *(archived)*

- [x] **setup test coverage for typescript and rust**
  *Link: [./conductor/archive/test-coverage-ts-rust/](./conductor/archive/test-coverage-ts-rust/)*
  *(Archived)*

- [x] **Phase 3 SteamCMD Integration:** SteamCMD integration (server install/update)
  *Link: [./conductor/archive/phase3_steamcmd_integration_20260408/](./conductor/archive/phase3_steamcmd_integration_20260408/)*
  *(Archived)*

- [x] **Lint Stage Enhancement:** Update lint stage with rust code quality check and script to check if files have more than 500 lines of code
  *Link: [./conductor/archive/lint-stage-enhancement_20260408/](./conductor/archive/lint-stage-enhancement_20260408/)*
  *(Archived)*

- [x] **Phase 2 — Server Lifecycle Management:** Start/stop ARK servers, live console output, RCON player list
  *Link: [./conductor/archive/phase2_server_lifecycle_20260408/](./conductor/archive/phase2_server_lifecycle_20260408/)*
  *(Archived)*

- [x] **Phase 1 — Server Profile Management:** Profile creation wizard, visual + raw INI editor (split view), JSON storage, CRUD, validation
  *Link: [./conductor/archive/phase1_server_profile_mgmt_20260408/](./conductor/archive/phase1_server_profile_mgmt_20260408/)*
  *(Archived)*

---

## How to Create a Track

Use the `conductor-new-track` skill to plan and create a new track:

```bash
/conductor:new-track
# or
conductor new-track
```

This will:
1. Define the track scope and acceptance criteria
2. Generate track-specific spec documents
3. Create the track directory under `conductor/tracks/<name>/`
4. Update this registry

---

## Track Structure

Each track directory contains:
- `plan.md` — Detailed implementation plan
- `spec.md` — Track-specific specifications
- Any supporting files

---

## Phase Definitions

| Phase | Description |
|-------|-------------|
| Phase 0 | Project setup (scaffolding, tooling, initial architecture) |
| Phase 1 | Core server management (start/stop/status) |
| Phase 2 | Server configuration (INI editing, profiles) |
| Phase 3 | SteamCMD integration (server install/update) |
| Phase 4 | Polish (UI refinement, error handling, backups) |
| Phase 5 | Linux support |
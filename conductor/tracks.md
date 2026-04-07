# Tracks Registry

## Overview
Tracks are major phases of development. Each track has its own plan and working directory.

---

## Track Listing

*(No tracks defined yet — use `conductor-new-track` to create the first one)*

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

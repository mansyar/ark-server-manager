# Phase 1 — Server Profile Management

## Track Context

| Field | Value |
|-------|-------|
| **Track ID** | `phase1_server_profile_mgmt_20260408` |
| **Status** | Pending |
| **Type** | Feature |
| **Created** | 2026-04-08 |

## What This Track Covers

1. **Rust Backend** — Tauri commands for profile CRUD + INI parsing
2. **State** — Zustand `profilesSlice` with async Tauri invoke
3. **UI: Profile List** — Card grid with create/edit/delete actions
4. **UI: Creation Wizard** — 4-step modal with ARK defaults
5. **UI: INI Editor** — Visual/Raw/Split modes with sync
6. **Validation** — Zod schemas, inline errors, block save

## Key Files

- `spec.md` — Full specification
- `plan.md` — Implementation plan (41 tasks across 10 phases)
- `metadata.json` — Track metadata

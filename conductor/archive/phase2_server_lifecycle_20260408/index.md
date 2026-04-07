# Phase 2 — Server Lifecycle Management

## Quick Links
- [Spec](./spec.md)
- [Plan](./plan.md)

## Context
- **Prerequisite:** Phase 1 (profile storage, INI generation)
- **Next:** Phase 3 (SteamCMD integration)
- **Product:** Ark Server Manager (ASM)

## Key Dependencies
- `src-tauri/src/` — Rust backend (Tauri commands)
- `src/stores/` — Zustand stores
- `src/components/` — React UI components

## Rust Crates Needed
- `sysinfo` — process enumeration
- `tokio` — async process management
- `rcon` — RCON protocol client
- `strip-ansi` — ANSI escape code stripping

# Phase 0 — Project Setup

## 1. Overview

**Track ID:** `phase0_project_setup_20250408`  
**Type:** Chore  
**Summary:** Scaffold a production-ready Tauri 2.x + React 19 + Vite + TypeScript project with pnpm, configure tooling (ESLint, Prettier, TypeScript, Clippy, husky), establish folder structure, set up logging, and verify the empty shell builds and runs correctly.

---

## 2. Functional Requirements

| # | Requirement |
|---|-------------|
| F1 | Initialize Tauri 2.x app with React 19 + TypeScript + Vite using official `create-tauri-app` |
| F2 | Use **pnpm** as the package manager |
| F3 | Preserve any existing files in the project root (no deletion) |
| F4 | Use **standard folder structure**: `src-tauri/` (Rust), `src/` (React) |
| F5 | Configure ESLint + Prettier with React 19 / TypeScript rules |
| F6 | Configure TypeScript (`strict: true`, path aliases) |
| F7 | Configure husky v9 + lint-staged for pre-commit linting |
| F8 | Configure Rust `clippy` in `src-tauri/` |
| F9 | Set up `tracing` + `tracing-subscriber` with JSON fmt in Rust backend |
| F10 | Wire Zustand store skeleton (empty store, no slices yet) |
| F11 | Verify dark theme + shadcn/ui components render in React |
| F12 | Confirm all commands pass in CI mode: `CI=true pnpm run check`, `cargo clippy`, `pnpm test` |

---

## 3. Non-Functional Requirements

| # | Requirement |
|---|-------------|
| N1 | Project must be runnable via `pnpm tauri dev` after setup |
| N2 | Build must produce a working `.exe` via `pnpm tauri build` |
| N3 | No console errors on initial load |
| N4 | husky hooks must not block `git commit` on lint pass |
| N5 | All tooling versions pinned in `package.json` and `Cargo.lock` |

---

## 4. Acceptance Criteria

| # | Criterion |
|---|-----------|
| AC1 | `pnpm create tauri-app` succeeds with React 19 + TypeScript + Vite |
| AC2 | `pnpm install` completes without errors |
| AC3 | `pnpm tauri dev` launches the app window without errors |
| AC4 | `pnpm run check` (lint + typecheck) passes with zero errors |
| AC5 | `cargo clippy` passes with zero warnings in `src-tauri/` |
| AC6 | `pnpm test` runs and passes (empty test suite is valid) |
| AC7 | `pnpm tauri build` produces a runnable `.exe` in `src-tauri/target/release/` |
| AC8 | husky pre-commit hook runs `pnpm run check` and blocks bad commits |
| AC9 | Logs are written to `%APPDATA%\ArkServerManager\logs\` on startup |
| AC10 | No existing project files are deleted or overwritten |

---

## 5. Out of Scope

- Any feature code beyond empty shell verification
- Server management logic (Phase 1 territory)
- INI editing, SteamCMD, process spawning
- Installer configuration

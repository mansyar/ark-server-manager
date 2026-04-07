# Phase 0 — Project Setup: Implementation Plan

## Phase 0.1: Scaffold Project

- [ ] **0.1.1** Verify prerequisites (pnpm, Rust, Node.js versions) via terminal checks
- [ ] **0.1.2** Run `pnpm create tauri-app@latest` with React 19 + TypeScript + Vite template into the project root
- [ ] **0.1.3** Verify folder structure created: `src-tauri/`, `src/`, `index.html`, `package.json`
- [ ] **0.1.4** Remove any generated lockfiles that conflict (prefer pnpm-lock.yaml)
- [ ] **0.1.5** Verify `pnpm install` completes without errors
- [ ] **0.1.6** Confirm `pnpm tauri dev` launches a window (headless test: window spawns, no crash)

## Phase 0.2: Configure TypeScript & React

- [ ] **0.2.1** Update `tsconfig.json` with `strict: true`, `jsx: preserve`, and path aliases (`@/*` → `src/*`)
- [ ] **0.2.2** Install React 19 types if not already bundled
- [ ] **0.2.3** Configure `vite.config.ts` with path aliases and React 19 plugin settings
- [ ] **0.2.4** Run `pnpm run check` (tsc + eslint) — fix any errors

## Phase 0.3: Configure ESLint + Prettier

- [ ] **0.3.1** Install ESLint with React 19 + TypeScript + Vite configs
- [ ] **0.3.2** Install and configure Prettier with sensible defaults
- [ ] **0.3.3** Add `.prettierrc` and `.eslintrc` files
- [ ] **0.3.4** Configure `lint-staged` in `package.json` to run `pnpm run check` on staged files
- [ ] **0.3.5** Verify `pnpm run check` passes cleanly

## Phase 0.4: Configure Husky Git Hooks

- [ ] **0.4.1** Initialize husky v9 in the project
- [ ] **0.4.2** Create pre-commit hook that runs lint-staged
- [ ] **0.4.3** Verify hook fires on `git commit` and blocks bad commits
- [ ] **0.4.4** Verify hook allows clean commits to pass

## Phase 0.5: Configure Rust Tooling

- [ ] **0.5.1** Verify `Cargo.toml` has latest stable Rust deps
- [ ] **0.5.2** Configure `.clippy.toml` (or clippy.toml in `src-tauri/`) with warnings-as-errors
- [ ] **0.5.3** Run `cargo clippy --all-targets --all-features` — zero warnings
- [ ] **0.5.4** Fix any clippy warnings

## Phase 0.6: Set Up Logging

- [ ] **0.6.1** Add `tracing` + `tracing-subscriber` + `tracing-appender` to `src-tauri/Cargo.toml`
- [ ] **0.6.2** Write logging initialization code in `src-tauri/src/main.rs` (JSON fmt, log dir via env)
- [ ] **0.6.3** Verify logs are written to `%APPDATA%\ArkServerManager\logs\`
- [ ] **0.6.4** Verify startup log entry appears on first `pnpm tauri dev` run

## Phase 0.7: Wire Zustand Store Skeleton

- [ ] **0.7.1** Install Zustand in `src/`
- [ ] **0.7.2** Create empty store at `src/stores/appStore.ts`
- [ ] **0.7.3** Connect store to React app entry point
- [ ] **0.7.4** Verify app renders without store errors

## Phase 0.8: Configure shadcn/ui + Dark Theme

- [ ] **0.8.1** Initialize shadcn/ui with dark theme ( `--theme dark` )
- [ ] **0.8.2** Add 1-2 basic components (e.g., Button, Card) to verify rendering
- [ ] **0.8.3** Verify dark theme + ARK amber/orange accent renders in dev mode
- [ ] **0.8.4** Verify shadcn components are accessible

## Phase 0.9: CI Verification

- [ ] **0.9.1** Run `CI=true pnpm run check` — must pass with zero errors
- [ ] **0.9.2** Run `cargo clippy --all-targets --all-features` — zero warnings
- [ ] **0.9.3** Run `pnpm test` — passes (empty suite is OK)
- [ ] **0.9.4** Run `pnpm tauri build` — produces `.exe` in `src-tauri/target/release/`

## Phase 0.10: Finalize

- [ ] **0.10.1** Verify no existing project files in `conductor/` were modified
- [ ] **0.10.2** Commit all changes with `chore(phase0): Scaffold project setup`
- [ ] **0.10.3** Attach task summary as git note to commit
- [ ] **0.10.4** Update this plan: mark all tasks `[x]` with commit SHAs

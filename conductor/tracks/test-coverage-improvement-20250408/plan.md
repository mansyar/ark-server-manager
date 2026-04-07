# Implementation Plan: test-coverage-improvement-20250408

## Overview
Add more tests for `src/` (TypeScript/React) and `src-tauri/` (Rust) to achieve project coverage threshold (80% line, 70% branch).

## Status: [ ]

---

## Phase A: TypeScript Tests (src/)

### Task A1: [ ] Add tests for `src/lib/utils.ts`
- **File:** `src/lib/__tests__/utils.test.ts`
- **Coverage target:** cn() function with various class value combinations

### Task A2: [ ] Add tests for `src/types/profile.ts`
- **File:** `src/types/__tests__/profile.test.ts`
- **Coverage target:** ARK_MAPS array, ArkMap type, interface structures

### Task A3: [ ] Add tests for `src/types/server.ts`
- **File:** `src/types/__tests__/server.test.ts`
- **Coverage target:** ServerStatus type, interfaces

### Task A4: [ ] Add tests for `src/stores/appStore.ts`
- **File:** `src/stores/__tests__/appStore.test.ts`
- **Coverage target:** useAppStore creation and basic state

### Task A5: [ ] Add tests for `src/stores/profilesStore.ts`
- **File:** `src/stores/__tests__/profilesStore.test.ts`
- **Coverage target:** Store actions: loadProfiles, createProfile, updateProfile, deleteProfile

### Task A6: [ ] Add tests for `src/stores/serverLifecycleStore.ts`
- **File:** `src/stores/__tests__/serverLifecycleStore.test.ts`
- **Coverage target:** useServerStore actions (getStatus, refreshStatus, startServer, etc.)

---

## Phase B: Rust Tests (src-tauri/)

### Task B1: [ ] Add unit tests for `server_state.rs`
- **File:** `src-tauri/src/services/server_state.rs` (add `#[cfg(test)]` module)
- **Coverage target:** ServerState transitions and validation

### Task B2: [ ] Add unit tests for `steam_errors.rs`
- **File:** `src-tauri/src/services/steam_errors.rs` (add `#[cfg(test)]` module)
- **Coverage target:** Error message formatting

### Task B3: [ ] Add integration test for profile deserialization
- **File:** `src-tauri/tests/profile_integration_test.rs`
- **Coverage target:** Profile JSON parsing with various field combinations

### Task B4: [ ] Add tests for `update_manager.rs`
- **File:** `src-tauri/src/services/update_manager.rs` (add `#[cfg(test)]` module)
- **Coverage target:** Update check logic

---

## Phase C: Coverage Verification

### Task C1: [ ] Run TypeScript coverage
- **Command:** `npm run test:coverage`
- **Threshold:** Line ≥80%, Branch ≥70%

### Task C2: [ ] Run Rust coverage
- **Command:** `cargo test` + `cargo tarpaulin` (if available)
- **Threshold:** Line ≥80%, Branch ≥70%

### Task C3: [ ] Fix any coverage gaps
- **If coverage below threshold:** Add more tests or refactor untestable code
- **If refactoring needed:** Document in review

---

## Verification Steps

1. All `npm test` must pass (TypeScript)
2. All `cargo test` must pass (Rust)
3. Coverage reports show ≥80% line, ≥70% branch for both TS and Rust
4. No new lint errors introduced

---

## Dependencies
- Vitest configured in package.json
- @testing-library/react installed
- Rust testing via cargo

## Notes
- Mock Tauri invoke calls using vi.mock() for TypeScript store tests
- Rust tests use #[cfg(test)] modules within each .rs file
- Component tests use @testing-library/react
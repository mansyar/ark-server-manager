# Specification: Test Coverage Improvement Track

## 1. Overview

**Track ID:** `test-coverage-improvement-v2_20250408`
**Description:** Add more tests to src/ (business logic) and src-tauri/ to achieve workflow.md target coverage (>80%)
**Type:** Testing / Quality Improvement

## 2. Problem Statement

Current test coverage analysis shows:

| Module | Current Status | Target |
|--------|---------------|--------|
| `src/stores/serverLifecycleStore.ts` | ~10% (most functions untested) | >80% |
| `src/stores/profilesStore.ts` | ~50% (partial) | >80% |
| `src/types/server.ts` | 0% (no test coverage) | >80% |
| `src/types/profile.ts` | ~90% (covered) | Maintain |
| `src/lib/utils.ts` | ~50% (partial) | >80% |
| `src/lib/validation.ts` | ~90% (covered) | Maintain |
| `src/components/` | Partially covered | >80% |
| `src-tauri/` (Rust) | Well covered (~80%) | Maintain |

The workflow.md mandates >80% code coverage for all modules. Several TypeScript modules are below this threshold.

## 3. Functional Requirements

### 3.1 TypeScript Store Tests

#### 3.1.1 serverLifecycleStore Tests
- [ ] `initListeners` — mock Tauri event listeners, verify unlisteners array populated
- [ ] `cleanupListeners` — verify all unlisteners called and array cleared
- [ ] `getStatus` — return default 'Stopped' for unknown profile, actual status for known
- [ ] `refreshStatus` — invoke backend, update status in store on success
- [ ] `refreshStatus` error path — catch exception, log to console.error
- [ ] `startServer` — set isStarting true, optimistically set status to 'Starting', call invoke
- [ ] `startServer` error path — catch exception, reset status to 'Stopped', isStarting to false, set error
- [ ] `stopServer` — set isStopping true, call invoke
- [ ] `stopServer` error path — catch exception, set isStopping to false, set error
- [ ] `restartServer` — call stopServer, wait 2000ms, call startServer
- [ ] `validateInstall` — invoke backend, store result
- [ ] `validateInstall` error path — create error result, store it, return it
- [ ] `getConsoleBuffer` — invoke backend, update buffer in store, return it
- [ ] `getConsoleBuffer` error path — log error, return existing buffer
- [ ] `clearConsoleBuffer` — set buffer to empty array for profile
- [ ] `addConsoleLine` — append line to buffer, keep last 1000 lines
- [ ] `addConsoleLine` when buffer doesn't exist — create new buffer with line
- [ ] `getPlayers` — return players array or empty array for unknown profile
- [ ] `setActiveServerProfile` — set activeServerProfile state
- [ ] Event: server-started — update status to Running, set handle, set isStarting false, clear error
- [ ] Event: server-stopped — update status to Stopped, set isStopping false
- [ ] Event: status-changed — update status for profile
- [ ] Event: server-crashed — update status to Crashed, set error, reset isStarting/isStopping
- [ ] Event: console-output — call addConsoleLine with event payload
- [ ] Event: player-list-updated — update players, update lastPlayerUpdate timestamp

#### 3.1.2 profilesStore Tests
- [ ] `loadProfiles` — invoke list_profiles, set profiles, set isLoading false
- [ ] `loadProfiles` error path — set error message, set isLoading false
- [ ] `createProfile` — invoke save_profile, reload profiles, close wizard, set isLoading false
- [ ] `createProfile` error path — set error, set isLoading false, leave wizard open
- [ ] `updateProfile` — invoke save_profile, reload profiles, set activeProfile
- [ ] `updateProfile` error path — set error, set isLoading false
- [ ] `deleteProfile` — invoke delete_profile, reload profiles, clear activeProfile, close editor
- [ ] `deleteProfile` error path — set error, set isLoading false
- [ ] `setActiveProfile` — set activeProfile state
- [ ] `setWizardOpen` — set wizardOpen state
- [ ] `setEditorOpen` — set editorOpen state

### 3.2 TypeScript Types Tests

#### 3.2.1 server.ts Tests
- [ ] All type definitions have at least one test that exercises them
- [ ] ServerStatus union type — test each status value ('Stopped' | 'Starting' | 'Running' | 'Crashed')
- [ ] ServerHandle type — verify profile_name, pid, port fields
- [ ] ConsoleLine type — verify timestamp, message, profile_name fields
- [ ] PlayerInfo type — verify id, name, tribe, level fields
- [ ] ValidationResult type — verify is_valid, message, ark_exe_path fields

### 3.3 Component Tests (as needed)
- [ ] ServerControls component tests (already exists, verify coverage >80%)
- [ ] Add tests for missing interactions

### 3.4 Utility Tests
- [ ] `src/lib/utils.ts` — any untested utility functions

## 4. Non-Functional Requirements

- All new tests must pass (`npm test`)
- Coverage must meet or exceed >80% threshold per workflow.md
- Tests must follow TDD approach: write failing test first, then implement
- Tests must be isolated (mock external dependencies like Tauri invoke/listen)
- No test file should exceed 500 lines (per code styleguide)

## 5. Acceptance Criteria

| Criterion | Measure |
|-----------|---------|
| `serverLifecycleStore` coverage >80% | Vitest coverage report |
| `profilesStore` coverage >80% | Vitest coverage report |
| `server.ts` coverage >80% | Vitest coverage report |
| All tests passing | `npm test` exits 0 |
| No new lint errors | `npm run check` passes |

## 6. Out of Scope

- Rust test coverage (already passing >80%)
- E2E tests (separate track)
- UI snapshot tests
- Modifying production code (only tests added)

## 7. Dependencies

- Vitest (existing)
- @testing-library/react (existing)
- @tauri-apps/api/core mock (use manual mocks)

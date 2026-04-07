# Implementation Plan: Test Coverage Improvement

## Track: test-coverage-improvement-v2_20250408

## Phase 1: Setup Test Infrastructure

- [ ] 1.1 Review existing test files and Vitest configuration
- [ ] 1.2 Verify current coverage baseline with `npm run test:coverage`
- [ ] 1.3 Create manual mocks for Tauri APIs (`invoke`, `listen`)

## Phase 2: serverLifecycleStore Tests

- [ ] 2.1 Create `src/stores/__tests__/serverLifecycleStore.test.ts`
- [ ] 2.2 Write tests for `initListeners` (mock Tauri events)
- [ ] 2.3 Write tests for `cleanupListeners`
- [ ] 2.4 Write tests for `getStatus`
- [ ] 2.5 Write tests for `refreshStatus` (success + error paths)
- [ ] 2.6 Write tests for `startServer` (success + error paths)
- [ ] 2.7 Write tests for `stopServer` (success + error paths)
- [ ] 2.8 Write tests for `restartServer`
- [ ] 2.9 Write tests for `validateInstall` (success + error paths)
- [ ] 2.10 Write tests for `getConsoleBuffer` (success + error paths)
- [ ] 2.11 Write tests for `clearConsoleBuffer`
- [ ] 2.12 Write tests for `addConsoleLine` (with existing buffer, without buffer)
- [ ] 2.13 Write tests for `getPlayers`
- [ ] 2.14 Write tests for `setActiveServerProfile`
- [ ] 2.15 Write tests for all event handlers (server-started, server-stopped, status-changed, server-crashed, console-output, player-list-updated)
- [ ] 2.16 Run tests, verify all pass
- [ ] 2.17 Verify coverage >80% for serverLifecycleStore

## Phase 3: profilesStore Tests

- [ ] 3.1 Create `src/stores/__tests__/profilesStore.test.ts`
- [ ] 3.2 Write tests for `loadProfiles` (success + error paths)
- [ ] 3.3 Write tests for `createProfile` (success + error paths)
- [ ] 3.4 Write tests for `updateProfile` (success + error paths)
- [ ] 3.5 Write tests for `deleteProfile` (success + error paths)
- [ ] 3.6 Write tests for `setActiveProfile`
- [ ] 3.7 Write tests for `setWizardOpen`
- [ ] 3.8 Write tests for `setEditorOpen`
- [ ] 3.9 Run tests, verify all pass
- [ ] 3.10 Verify coverage >80% for profilesStore

## Phase 4: Type Tests (server.ts)

- [ ] 4.1 Create `src/types/__tests__/server.test.ts`
- [ ] 4.2 Write tests exercising all type definitions (ServerStatus, ServerHandle, ConsoleLine, PlayerInfo, ValidationResult)
- [ ] 4.3 Run tests, verify all pass
- [ ] 4.4 Verify coverage >80% for server.ts

## Phase 5: Verification

- [ ] 5.1 Run full test suite: `npm test`
- [ ] 5.2 Run full coverage: `npm run test:coverage`
- [ ] 5.3 Verify all stores/components >80%
- [ ] 5.4 Verify no lint errors: `npm run check`
- [ ] 5.5 Update test coverage summary in this plan

## Phase 6: Cleanup & Commit

- [ ] 6.1 Commit all new test files
- [ ] 6.2 Commit updated plan.md
- [ ] 6.3 Update tracks.md registry

# Specification: Test Coverage Improvement for src/ and src-tauri/

## 1. Overview

**Track:** test-coverage-improvement-20250408  
**Description:** Add more tests for `src/` (TypeScript/React) and `src-tauri/` (Rust) to achieve project coverage threshold  
**Target Coverage:** 80% line, 70% branch (per workflow standard)

## 2. Functional Requirements

### 2.1 TypeScript/React Tests (`src/`)
- Identify untested source files in `src/lib/`, `src/stores/`, `src/components/`
- Add unit tests for utility functions in `src/lib/utils.ts`
- Add unit tests for Zustand stores (`src/stores/*.ts`)
- Add component tests for key UI components
- Existing test: `src/lib/__tests__/validation.test.ts`

### 2.2 Rust Tests (`src-tauri/`)
- Identify untested modules in `src-tauri/src/`
- Add unit tests for services: `ini_conversion.rs`, `retry.rs`, `server_state.rs`
- Add tests for models: `profile.rs`
- Add integration tests in `src-tauri/tests/`
- Existing: No test files in `src-tauri/tests/`

### 2.3 Coverage Enforcement
- Run coverage tools for both TypeScript (Vitest + V8) and Rust (cargo-tarpaulin or cargo-nextest)
- Ensure all new code meets thresholds before completion

## 3. Non-Functional Requirements

- All tests must pass (no flaky tests)
- Test files must follow naming: `*.test.ts` for TS, `*_test.rs` for Rust
- Line count limits apply to test files (max 500 lines per file)
- Tests should be deterministic and isolated

## 4. Acceptance Criteria

| # | Criterion | Verification |
|---|-----------|--------------|
| 1 | `src/lib/utils.ts` has unit tests | Coverage report shows >80% |
| 2 | `src/stores/` tests cover all store actions | Tests for each store action |
| 3 | `src/components/` key components have tests | ServerControls, PlayerList, etc. |
| 4 | Rust services have unit tests | `cargo test` passes |
| 5 | Rust models have tests | Tests for Profile serialization/deserialization |
| 6 | Combined coverage meets threshold | Line ≥80%, Branch ≥70% |

## 5. Out of Scope

- E2E tests
- Mocking external system calls (SteamCMD, ARK server processes)
- Code coverage for generated files (shadcn/ui components)
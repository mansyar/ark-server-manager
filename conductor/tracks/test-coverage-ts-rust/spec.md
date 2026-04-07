# Specification: Test Coverage Setup for TypeScript and Rust

## 1. Overview

**Track:** Setup test coverage for TypeScript and Rust  
**Goal:** Configure test coverage tooling and achieve >80% line coverage and >70% branch coverage for the Ark Server Manager codebase.  
**Scope:** TypeScript (React frontend) and Rust (Tauri backend) test coverage setup.

---

## 2. Functional Requirements

### 2.1 TypeScript Coverage Setup

| # | Requirement | Details |
|---|-------------|---------|
| TS-1 | Install Vitest with coverage | Configure Vitest as the test runner with V8 coverage provider |
| TS-2 | Configure coverage thresholds | Line: 80%, Branch: 70% |
| TS-3 | Coverage reports | Generate HTML and JSON coverage reports |
| TS-4 | Run coverage on existing tests | Verify current test suite meets thresholds |
| TS-5 | Report output location | `coverage/ts/` directory |

**Test Files Location:** `src/components/__tests__/` and any `*.test.ts` / `*.spec.ts` files  
**Source Files:** `src/` (excluding `src/main.tsx`, `src/vite-env.d.ts`, and asset files)

### 2.2 Rust Coverage Setup

| # | Requirement | Details |
|---|-------------|---------|
| RS-1 | Install tarpaulin | Configure cargo-tarpaulin for coverage |
| RS-2 | Configure coverage thresholds | Line: 80%, Branch: 70% |
| RS-3 | Coverage reports | Generate HTML and XML (Cobertura) reports |
| RS-4 | Run coverage on existing tests | Verify current test suite meets thresholds |
| RS-5 | Report output location | `coverage/rs/` directory |

**Test Files Location:** `src-tauri/tests/` and any `*_test.rs` files  
**Source Files:** `src-tauri/src/` (excluding generated/main.rs boilerplate)

### 2.3 Unified Coverage Workflow

| # | Requirement | Details |
|---|-------------|---------|
| UW-1 | Single command | `npm run test:coverage` runs both TS and Rust coverage |
| UW-2 | Combined report | Summary of both TypeScript and Rust coverage |
| UW-3 | Fail on threshold breach | Exit code non-zero if coverage below threshold |

---

## 3. Non-Functional Requirements

| # | Requirement |
|---|-------------|
| NF-1 | No CI integration (local-only verification) |
| NF-2 | Coverage must pass on existing test suite without adding new tests |
| NF-3 | Reports must be viewable in browser (HTML output) |
| NF-4 | Tooling must be non-destructive to existing code and tests |
| NF-5 | Configuration must be reproducible across machines |

---

## 4. Acceptance Criteria

| # | Criterion | Verification |
|---|-----------|--------------|
| AC-1 | `npm run test:coverage` runs both TypeScript and Rust coverage | Execute and verify output |
| AC-2 | TypeScript coverage report generated at `coverage/ts/index.html` | Verify file exists after run |
| AC-3 | Rust coverage report generated at `coverage/rs/index.html` | Verify file exists after run |
| AC-4 | Line coverage >= 80% for TypeScript | Check report output |
| AC-5 | Branch coverage >= 70% for TypeScript | Check report output |
| AC-6 | Line coverage >= 80% for Rust | Check report output |
| AC-7 | Branch coverage >= 70% for Rust | Check report output |
| AC-8 | Command exits with non-zero if thresholds not met | Verify failure behavior |

---

## 5. Out of Scope

- GitHub Actions / CI integration
- Adding new tests (tooling setup only)
- E2E test coverage
- Coverage badge generation
- SonarQube or other external service integration

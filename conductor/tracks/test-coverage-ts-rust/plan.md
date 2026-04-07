# Implementation Plan: Test Coverage Setup for TypeScript and Rust

## Overview
Configure test coverage tooling for Ark Server Manager (TypeScript React frontend + Rust Tauri backend) with >80% line and >70% branch coverage targets.

---

## Phase 1: TypeScript Coverage Setup

### Task 1.1: Install and Configure Vitest Coverage
- [ ] Install Vitest coverage dependencies (vite-plugin-coverage-v8 or vitest's built-in coverage)
- [ ] Configure `vitest.config.ts` with coverage settings
- [ ] Set thresholds: lines 80%, branches 70%
- [ ] Configure HTML and JSON report generation
- [ ] Set report output directory to `coverage/ts/`

### Task 1.2: Verify TypeScript Coverage
- [ ] Run coverage on existing test files
- [ ] Verify coverage meets thresholds
- [ ] If coverage below threshold: identify uncovered areas
- [ ] Document findings in coverage report

---

## Phase 2: Rust Coverage Setup

### Task 2.1: Install and Configure cargo-tarpaulin
- [ ] Install cargo-tarpaulin (`cargo install cargo-tarpaulin`)
- [ ] Configure `Cargo.toml` with tarpaulin settings if needed
- [ ] Set thresholds: lines 80%, branches 70%
- [ ] Configure HTML and XML (Cobertura) report generation
- [ ] Set report output directory to `coverage/rs/`

### Task 2.2: Verify Rust Coverage
- [ ] Run coverage on existing test files
- [ ] Verify coverage meets thresholds
- [ ] If coverage below threshold: identify uncovered areas
- [ ] Document findings in coverage report

---

## Phase 3: Unified Coverage Workflow

### Task 3.1: Create Unified Coverage Script
- [ ] Add `npm run test:coverage` script to `package.json`
- [ ] Script runs TypeScript coverage first, then Rust coverage
- [ ] Script exits non-zero if either coverage fails threshold
- [ ] Print combined summary to console

### Task 3.2: Verify Full Workflow
- [ ] Run `npm run test:coverage`
- [ ] Verify both reports generated
- [ ] Verify exit code behavior on success/failure
- [ ] Clean up any temporary files

---

## Verification Checklist

- [ ] `coverage/ts/index.html` exists and shows >= 80% line, >= 70% branch
- [ ] `coverage/rs/index.html` exists and shows >= 80% line, >= 70% branch
- [ ] `npm run test:coverage` completes successfully
- [ ] `npm run test:coverage` exits non-zero on threshold breach

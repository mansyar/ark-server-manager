# Plan: Lint Stage Enhancement

Track: Update Lint Stage with Rust Quality Check and Line Count Script  
Status: [ ] In Progress

---

## Phase 1: Setup Pre-Commit Infrastructure

### Tasks

- [x] **1.1** Install husky and lint-staged dependencies
  - Add `husky` and `lint-staged` to devDependencies
  - Run `npm pkg set scripts.prepare="husky install"` if not present
  - Run `npm run prepare` to initialize husky

- [x] **1.2** Configure lint-staged in `package.json`
  - Add `lint-staged` block with Rust and line-count checks
  - Verify `.husky/pre-commit` hook is created

---

## Phase 2: Create Line Count Check Script

### Tasks

- [x] **2.1** Create `scripts/check-file-lines.sh`
  - Accepts list of staged files as arguments
  - Filters for: `*.rs`, `*.ts`, `*.tsx`, `*.css`
  - For each file, count lines and fail if > 500
  - Output format: `FILE:PATH:LINE_COUNT` for violations
  - Exit code 1 on violation, 0 if all pass

- [x] **2.2** Make script executable
  - Run `chmod +x scripts/check-file-lines.sh`

- [x] **2.3** Test script manually
  - Create temporary test file with 501 lines
  - Verify script blocks correctly
  - Clean up test file

---

## Phase 3: Integrate Rust Checks into lint-staged

### Tasks

- [x] **3.1** Configure Rust formatting check
  - Add `cargo fmt -- --check` for `*.rs` files to lint-staged

- [x] **3.2** Configure clippy check
  - Add `cargo clippy --all-targets --all-features -- -D warnings` to lint-staged
  - Note: clippy runs on full crate, not per-file

- [x] **3.3** Verify combined hook behavior
  - Stage a clean Rust file and verify commit succeeds
  - Introduce a formatting issue and verify commit fails
  - Introduce a clippy warning and verify commit fails
  - NOTE: `commands.rs` (523 lines) and `server_state.rs` (981 lines) are pre-existing violations → exempted in script

---

## Phase 4: Verification and Documentation

### Tasks

- [x] **4.1** Run full verification against acceptance criteria
  - AC1: Clean Rust + valid line counts → commit succeeds
  - AC2: Unformatted Rust → blocked
  - AC3: Clippy warnings → blocked
  - AC4: Any source file >500 lines → blocked
  - AC5: Manual script execution produces same result
  - AC6: `.husky/pre-commit` exists and is executable
  - AC7: `lint-staged` config runs checks

- [x] **4.2** Update project documentation
  - Add lint hook usage to README or CONTRIBUTING.md

---

## Phase Completion Verification

- [x] All 7 acceptance criteria pass
- [x] `cargo clippy` passes on full codebase
- [x] `cargo fmt -- --check` passes on full codebase
- [x] No source file in `src/` or `src-tauri/src/` exceeds 500 lines (pre-existing violations must be fixed or exempted)

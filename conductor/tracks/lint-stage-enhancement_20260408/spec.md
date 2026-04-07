# Spec: Lint Stage Enhancement

**Track:** Update Lint Stage with Rust Quality Check and Line Count Script  
**Created:** 2026-04-08

---

## 1. Overview

This track enhances the local pre-commit hook to add:
1. **Rust code quality checks** via `clippy` + `rustfmt`
2. **File line count validation** — blocks commits if any source file exceeds 500 lines

---

## 2. Functional Requirements

### 2.1 Pre-Commit Hook Setup
- Integrate with **husky** for Git hooks management
- Use **lint-staged** to run checks on staged files
- Hook triggers on `git commit` (pre-commit stage)

### 2.2 Rust Code Quality Check
- Run `cargo fmt --check` to verify Rust formatting
- Run `cargo clippy --all-targets --all-features -- -D warnings` to catch code issues
- Both must pass for commit to proceed

### 2.3 File Line Count Check
- Script: `scripts/check-file-lines.sh`
- Scans staged files matching: `*.rs`, `*.ts`, `*.tsx`, `*.css`
- **Blocks commit** if any file exceeds 500 lines (exempted files excluded)
- Outputs offending file(s) with line count

---

## 3. Non-Functional Requirements

| Requirement | Detail |
|-------------|--------|
| Performance | Line check < 1s for typical staged files |
| Fail-fast | Stop on first failing check |
| Clear output | Report which file(s) and why commit was blocked |
| Idempotent | Script can be run multiple times safely |

---

## 4. Acceptance Criteria

| # | Criterion |
|---|-----------|
| AC1 | `git commit` with properly formatted Rust and valid line counts → **succeeds** |
| AC2 | `git commit` with unformatted Rust code → **blocked**, error shows format issues |
| AC3 | `git commit` with clippy warnings → **blocked**, error shows clippy output |
| AC4 | `git commit` with any `*.rs`, `*.ts`, `*.tsx`, `*.css` file > 500 lines → **blocked**, error shows file(s) and line counts |
| AC5 | Running `scripts/check-file-lines.sh` manually produces same result as hook |
| AC6 | husky hook is installed in `.husky/pre-commit` |
| AC7 | `lint-staged` configuration in `package.json` runs the checks |

---

## 5. Out of Scope

- CI pipeline modification (GitHub Actions)
- Auto-fixing of line count violations
- Non-source files (e.g., `.md`, `.json` config files)

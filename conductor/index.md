# Ark Server Manager — Project Index

## Overview
A native Windows desktop application for managing ARK: Survival Evolved dedicated servers with a modern, beginner-friendly UI.

**Stack:** Tauri 2.x + React 18 + TypeScript + Tailwind CSS + shadcn/ui

---

## Project Structure

```
ark-server-manager/
├── src/                      # React frontend
│   ├── components/           # UI components
│   ├── hooks/               # Custom React hooks
│   ├── stores/              # Zustand state stores
│   ├── lib/                 # Utilities, Tauri command wrappers
│   └── types/               # TypeScript types
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── commands/        # Tauri command handlers
│   │   ├── models/          # Data structures
│   │   ├── services/        # Business logic
│   │   └── error.rs         # Error types
│   └── Cargo.toml
├── conductor/               # Conductor project management
│   ├── product.md           # Product Definition
│   ├── product-guidelines.md # Product Guidelines
│   ├── tech-stack.md       # Tech Stack
│   ├── workflow.md          # Workflow
│   ├── code_styleguides/    # Code style guides
│   │   ├── typescript-react.md
│   │   └── rust.md
│   ├── tracks.md            # Tracks Registry
│   └── tracks/              # Track directories
└── plan.md                  # Implementation plan
```

---

## Conductor Files

| File | Purpose |
|------|---------|
| `product.md` | What to build and why |
| `product-guidelines.md` | UI/UX standards, error handling, data storage |
| `tech-stack.md` | Technology choices and rationale |
| `workflow.md` | Development workflow and quality gates |
| `code_styleguides/` | TypeScript+React and Rust coding standards |
| `tracks.md` | Registry of all tracks |
| `tracks/<name>/` | Individual track directories |

---

## Getting Started

### Setup
```bash
# Install dependencies
npm install && cargo build

# Start development
npm run dev          # React dev server
cargo tauri dev      # Tauri dev (runs React too)
```

### Before Committing
```bash
npm run check        # lint + type check
cargo clippy         # Rust linting
npm test             # Unit tests
```

---

## Workflow Summary

- **Coverage Target:** >80%
- **Commits:** Per-phase
- **Task Tracking:** Git notes attached to commits
- **Phase Checkpoints:** Tagged commits with verification reports

---

## Current Status

- [x] Conductor environment initialized
- [x] Product definition complete
- [x] Tech stack selected: Tauri + React
- [x] Code style guides created
- [x] Workflow configured
- [ ] Initial track to be defined

---

*Last updated: 2026-04-07*

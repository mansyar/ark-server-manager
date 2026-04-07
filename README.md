# Ark Server Manager (ASM)

A native Windows desktop application for managing ARK: Survival Evolved dedicated servers — enabling easy server creation, configuration, startup/shutdown, and monitoring without requiring command-line knowledge.

Built with Tauri 2, React 19, and TypeScript.

## Features (v0.1 — Profile Management)

Currently implemented:

- **Profile Management** — Create, edit, delete, and duplicate server profiles via a guided 4-step wizard
- **Map Selection** — Support for all major ARK maps (TheIsland, TheCenter, ScorchedEarth, Ragnarok, Aberration, Extinction, Genesis Part 1 & 2, Valguero, Hope, Lost Island, Fjordur, Turkey)
- **Server Configuration** — Configure difficulty (0–20), max players (1–100), admin password, and server port (27000–27015)
- **Extra Settings** — Add custom game settings and user settings per profile
- **Validation** — All inputs validated with Zod; safe defaults out of the box

## Roadmap

Planned for future releases:

| Priority | Feature |
|----------|---------|
| P0 | One-click Server Start/Stop with status monitoring |
| P0 | Visual INI file editor (Game.ini / GameUserSettings.ini) |
| P1 | Auto-update ARK server files via SteamCMD |
| P1 | Backup system (savegames and configs) |
| P1 | In-app console output viewer |
| P1 | Player list viewer |
| P2 | RCON command support |
| P2 | Scheduled restarts |

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | React 19 + TypeScript + Vite |
| Desktop | Tauri 2 (Rust backend) |
| UI | shadcn/ui + Tailwind CSS v4 |
| State | Zustand |
| Validation | Zod |
| Icons | Lucide React |

**Design:** Dark-theme-first with ARK-themed amber/orange accents.

## Data Storage

Profiles are stored as JSON files in `%APPDATA%\ArkServerManager\profiles\`.

## Development

```bash
# Install dependencies
npm install

# Run in development mode (frontend + Tauri)
npm run tauri dev

# Build for production
npm run tauri build

# Run frontend only
npm run dev

# Type check & lint
npm run check

# Run tests
npm run test
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Pre-Commit Hooks

This project uses husky + lint-staged to enforce code quality on every commit.

### What Runs on Commit

| File Type | Checks |
|-----------|--------|
| `*.rs` | `cargo fmt -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, line count check (max 500 lines) |
| `*.ts`, `*.tsx` | ESLint (auto-fix), Prettier (auto-fix), line count check (max 500 lines) |
| `*.css` | Line count check (max 500 lines) |

### Line Count Policy

Files exceeding 500 lines are blocked from commit. Pre-existing violations are exempted in `scripts/check-file-lines.sh`.

### Bypassing Hooks (Not Recommended)

```bash
git commit --no-verify  # Bypass all pre-commit hooks
```

Avoid this unless absolutely necessary — the hooks exist to maintain code quality.

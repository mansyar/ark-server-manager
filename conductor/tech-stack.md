# Tech Stack

## 1. Frontend (UI Layer)

| Component | Technology |
|-----------|------------|
| Framework | React 18+ with TypeScript |
| Bundler | Vite |
| Styling | Tailwind CSS |
| UI Components | shadcn/ui (Radix primitives) |
| Icons | Lucide React |
| State Management | Zustand (lightweight) |
| Font | Inter or Segoe UI Variable |

**Design Language:** Modern, dark-theme-first with ARK-themed amber/orange accents.

---

## 2. Backend (Server Layer)

| Component | Technology |
|-----------|------------|
| Framework | Tauri 2.x (Rust) |
| Language | Rust (latest stable) |
| Process Management | tokio async runtime |
| File Operations | serde + serde_json for config files |
| Logging | tracing + tracing-subscriber |
| Shell Commands | Rust std::process::Command |
| System Metrics | sysinfo (CPU, memory monitoring) |
| Backup Compression | zip crate |
| Notifications | tauri-plugin-notification |

**Responsibilities:**
- ARK server process spawning and lifecycle management
- INI file reading/writing (Game.ini, GameUserSettings.ini)
- SteamCMD invocation and progress tracking
- Windows Registry / Firewall rule management
- IPC with React frontend via Tauri commands
- Health monitoring (CPU, memory, process status)
- Backup creation with ZIP compression
- Crash detection and reporting

---

## 3. Build & Distribution

| Component | Technology |
|-----------|------------|
| Package Manager | pnpm (frontend), Cargo (Rust) |
| Tauri CLI | @tauri-apps/cli |
| Target | Windows x64 (.exe), Linux AppImage later |
| Signing | Tauri code signing (optional v1) |
| Installer | Tauri bundler (NSIS for Windows) |

---

## 4. Development Tools

| Component | Technology |
|-----------|------------|
| IDE | VS Code + rust-analyzer + Volar |
| Frontend Linting | ESLint + Prettier |
| Rust Linting | clippy |
| Git Hooks | husky + lint-staged |

---

## 5. External Dependencies

| Dependency | Purpose |
|------------|---------|
| SteamCMD | ARK server download and update (bundled or detected) |
| ARK Server Files | Installed separately by user or via SteamCMD |

**Note:** The application ships as a standalone `.exe`. SteamCMD and ARK server are separate installations managed by the app.

---

## 6. Data Storage

| Data | Location |
|------|----------|
| App Settings | `%APPDATA%\ArkServerManager\settings.json` |
| Server Profiles | `%APPDATA%\ArkServerManager\profiles\{name}.json` |
| Backups | `%APPDATA%\ArkServerManager\backups\` |
| Logs | `%APPDATA%\ArkServerManager\logs\` |
| App Data (Tauri) | Tauri default app data dir |

---

## 7. Technology Rationale

- **Tauri 2.x:** Smaller binaries than Electron, native performance, Rust backend for system operations
- **React + TypeScript:** Modern reactive UI with type safety
- **Tailwind + shadcn/ui:** Rapid development of modern, accessible UI components
- **Zustand:** Minimal boilerplate state management for a desktop app
- **tokio:** Async runtime for managing ARK server processes without blocking the UI

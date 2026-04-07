# Product Guidelines

## 1. UI/UX Standards

### Visual Design
- **Design Language:** Windows Fluent Design System (or closest native equivalent)
- **Color Palette:** Dark theme primary with ARK-themed accent (orange/amber highlights)
- **Typography:** Segoe UI Variable or Segoe UI (Windows system font)
- **Iconography:** Fluent Icons, outlined style
- **Spacing:** 4px base unit, 8px standard padding, 16px section gaps

### Layout Principles
- **Navigation:** Single-window with sidebar navigation (not multi-window)
- **Responsive:** Minimum window size 900x600, scales gracefully to 4K
- **Feedback:** All actions show loading indicators, success toasts, or error dialogs
- **Confirmation:** Destructive actions (delete server, stop server) require explicit confirmation

### Accessibility
- Full keyboard navigation support
- Minimum contrast ratio 4.5:1 for text
- Tooltips on all icon-only buttons
- Focus indicators visible

---

## 2. Feature Boundaries

### In Scope (Do)
- Local ARK server management (one machine at a time)
- Visual editing of GameUserSettings.ini and Game.ini
- One-click server start/stop
- Server profile switching
- SteamCMD integration for server updates
- In-app console log viewer
- Player list display

### Out of Scope (Don't)
- Remote server management
- Cluster/multi-server coordination
- Cloud or web-based features
- Non-Windows platforms
- Automatic mod installation (v1)
- RCON interactive console (v1 — read-only player list is ok)

---

## 3. Error Handling

| Scenario | Behavior |
|----------|----------|
| ARK server not installed | Show friendly onboarding with "Install Server" button |
| Port already in use | Dialog explaining which port and how to change it |
| Server crash | Show last 50 lines of crash log, offer "Open Logs Folder" |
| SteamCMD download fails | Retry button + link to manual download instructions |
| Config file corrupted | Offer to restore from auto-backup |
| Insufficient disk space | Pre-check before server creation with clear message |

---

## 4. User Onboarding

1. **First Launch:** Welcome screen explaining the app's purpose with "Get Started" button
2. **First Run:** Prompt to locate ARK server installation or trigger SteamCMD download
3. **Empty State:** When no server profile exists, show large "Create New Server" CTA
4. **Tooltips:** First-use tooltips on key UI elements (server name, port, admin password)

---

## 5. Data & Storage

- **Profiles:** JSON files in `%APPDATA%\ArkServerManager\profiles\`
- **Settings:** JSON in `%APPDATA%\ArkServerManager\settings.json`
- **Backups:** Zip archives in `%APPDATA%\ArkServerManager\backups\`
- **Logs:** Rolling log files in `%APPDATA%\ArkServerManager\logs\`
- **No cloud sync** — all data local by design

---

## 6. Naming Conventions

- Server profiles: User-defined friendly name
- Profile files: `{profile-name}.json`
- Backup naming: `{profile-name}_{ISO-timestamp}.zip`
- Log files: `server_{profile-name}_{date}.log`

---

## 7. Version Compatibility

- Target: ARK: Survival Evolved (current stable branch)
- Compatibility with ARK server updates is handled gracefully — app shows warning if known breaking change detected

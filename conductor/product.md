# Product Definition

## 1. Overview

**Project Name:** Ark Server Manager (ASM)  
**Type:** Native Windows Desktop Application  
**Core Summary:** A modern, beginner-friendly Windows desktop application for managing ARK: Survival Evolved dedicated servers — enabling easy server creation, configuration, startup/shutdown, and monitoring without requiring command-line knowledge.

**Target Users:**
- Home gamers who want to host private ARK servers
- Small clans or communities self-hosting ARK servers
- Beginner server operators with no prior command-line or sysadmin experience

---

## 2. Problem Statement

Currently, hosting an ARK server on Windows requires:
- Manually editing `.ini` configuration files
- Running command-line batch scripts
- Configuring Windows Firewall rules manually
- Using third-party tools that are outdated, ugly, or overly complex

This creates friction and frustration for non-technical users who just want to play ARK with friends.

---

## 3. Vision & Core Principles

**Vision:** One-click ARK server management with a modern, approachable interface that feels native to Windows.

**Principles:**
1. **Beginner-Friendly:** No configuration files, no command prompts — everything via UI
2. **Modern UI:** Uses Windows-native or modern toolkits (e.g., WPF, WinUI 3, or Avalonia)
3. **Safe by Default:** Sensible defaults, validation, and backup before changes
4. **Self-Contained:** No external dependencies for the user (bundles everything needed)

---

## 4. Core Features

| Priority | Feature |
|----------|---------|
| P0 | Server Profile Management (create, edit, delete, duplicate server installations) |
| P0 | One-click Server Start/Stop with status monitoring |
| P0 | Server Settings UI (game mode, difficulty, max players, admin password, etc.) |
| P0 | INI File Editor (visual + raw text mode) |
| P1 | Player list viewer (who is connected) |
| P1 | Basic console output viewer (in-app log display) |
| P1 | Auto-update ARK server files via Steam |
| P1 | Backup system (savegames, configs) |
| P2 | RCON command support |
| P2 | Scheduled restarts |
| P2 | Multiple server profiles management |

---

## 5. Success Criteria

1. User can install ARK server + launch a playable server within 5 minutes of opening the app
2. All common server settings are configurable via UI without touching config files
3. Server can be started/stopped with a single button click
4. Application is a standalone `.exe` with no runtime dependencies for end users
5. Application feels native to Windows 10/11 ( Fluent Design or equivalent)

---

## 6. Out of Scope (v1)

- macOS/Linux server support (Windows-only for v1)
- Hosting ARK server on remote machines (local-only)
- Cluster configuration (multiple linked servers)
- Mobile companion app
- Cloud save management

---

## 7. Technical Constraints

- Target OS: Windows 10/11 (64-bit)
- Technology: WPF, WinUI 3, or Avalonia (TBD in tech-stack phase)
- Must run without requiring .NET Runtime to be pre-installed (self-contained publish)
- ARK server installation via SteamCMD (bundled or detected)

# profile_path_config_ui_20260408

**Description:** Add UI for configuring SteamCMD and ARK server install paths in both the profile wizard and editor

**Spec:** `conductor/tracks/profile_path_config_ui_20260408/spec.md`  
**Plan:** `conductor/tracks/profile_path_config_ui_20260408/plan.md`

---

## Track Context

- **Type:** Feature
- **Created:** 2026-04-08
- **Status:** Planned

## Scope

- Add "Install Paths" step in `ProfileWizard.tsx`
- Add "Paths" section in `ProfileEditor.tsx`
- Create reusable `PathInput` UI component
- Rust backend path validation command
- Browse buttons + text input + validation

## Key Files

| File | Change |
|------|--------|
| `src/components/ui/PathInput.tsx` | New component |
| `src/components/profiles/ProfileEditor.tsx` | Add Paths section |
| `src/components/profiles/ProfileWizard.tsx` | Add Install Paths step |
| `src-tauri/src/commands/path_validation.rs` | New command |
| `src-tauri/src/commands/mod.rs` | Export new command |
| `src-tauri/src/models/profile.rs` | Already has fields |

## Dependencies

- Phase 1 (profile storage)
- Phase 2 (server lifecycle)
- Phase 3 (SteamCMD integration)
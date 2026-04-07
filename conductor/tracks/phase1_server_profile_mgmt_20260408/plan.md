# Phase 1 — Server Profile Management: Implementation Plan

## Phase 1.1: Rust Backend — Profile Storage Tauri Commands

- [ ] **1.1.1** Define `Profile` Rust struct with serde derive (name, map, difficulty, maxPlayers, adminPassword, port, extraSettings, schemaVersion)
- [ ] **1.1.2** Add Tauri commands: `list_profiles`, `load_profile`, `save_profile`, `delete_profile`
- [ ] **1.1.3** Implement `list_profiles` — reads `%APPDATA%\ArkServerManager\profiles\`, returns `Vec<ProfileMetadata>`
- [ ] **1.1.4** Implement `save_profile` — serializes Profile to JSON, writes to `{name}.json`
- [ ] **1.1.5** Implement `load_profile` — reads JSON, deserializes to Profile
- [ ] **1.1.6** Implement `delete_profile` — removes `{name}.json` from disk
- [ ] **1.1.7** Add error handling: missing directory → auto-create; file not found → `ProfileNotFound` error
- [ ] **1.1.8** Run `cargo clippy --all-targets --all-features` — zero warnings
- [ ] **1.1.9** Write unit tests for profile serde (round-trip: JSON → Profile → JSON)

## Phase 1.2: INI Parsing & Serialization

- [ ] **1.2.1** Add `ini` crate to `Cargo.toml` for INI parsing
- [ ] **1.2.2** Implement `ProfileToIni` trait: convert Profile → INI string (Game.ini + GameUserSettings.ini)
- [ ] **1.2.3** Implement `IniToProfile` trait: parse INI string → Profile
- [ ] **1.2.4** Handle ARK-specific INI sections: `[ServerSettings]`, `[GameUserSettings]`, `[SessionName]`
- [ ] **1.2.5** Add unit tests for INI ↔ Profile round-trip
- [ ] **1.2.6** Run `cargo clippy --all-targets --all-features` — zero warnings

## Phase 1.3: Zustand Store — Profiles Slice

- [ ] **1.3.1** Create `src/stores/profilesStore.ts` with Zustand slice
- [ ] **1.3.2** Define store shape: `profiles: Profile[]`, `activeProfile: Profile | null`, `isLoading: boolean`, `error: string | null`
- [ ] **1.3.3** Add actions: `loadProfiles`, `createProfile`, `updateProfile`, `deleteProfile`, `setActiveProfile`
- [ ] **1.3.4** Integrate with Tauri invoke calls (async, non-blocking)
- [ ] **1.3.5** Add error handling in store actions (set `error` state on failure)
- [ ] **1.3.6** Export typed hooks: `useProfilesStore`, `useActiveProfile`

## Phase 1.4: UI — Profile List View

- [ ] **1.4.1** Create `src/components/profiles/ProfileListView.tsx`
- [ ] **1.4.2** Display profile cards: name, map, status indicator, last modified date
- [ ] **1.4.3** Add "Create New Profile" button (opens wizard)
- [ ] **1.4.4** Add edit/delete quick-action buttons on each card
- [ ] **1.4.5** Empty state: large "Create Your First Server" CTA with illustration
- [ ] **1.4.6** Connect to `useProfilesStore` — load profiles on mount

## Phase 1.5: UI — Profile Creation Wizard

- [ ] **1.5.1** Create `src/components/profiles/ProfileWizard.tsx` as multi-step modal/drawer
- [ ] **1.5.2** Step 1: Profile name input with validation (unique, non-empty, filename-safe)
- [ ] **1.5.3** Step 2: Server settings (map selector, difficulty slider 0-20, max players 1-100)
- [ ] **1.5.4** Step 3: Admin password + confirm, port number (27000-27015 range)
- [ ] **1.5.5** Step 4: Review summary before saving
- [ ] **1.5.6** Progress indicator showing current step
- [ ] **1.5.7** Back/Next navigation; "Create Profile" button on final step
- [ ] **1.5.8** On submit: call `createProfile` Tauri command, close wizard, refresh list
- [ ] **1.5.9** Validation: inline errors per field, red border, block Next until valid

## Phase 1.6: UI — INI Editor (Split View)

- [ ] **1.6.1** Create `src/components/profiles/ProfileEditor.tsx`
- [ ] **1.6.2** Create `src/components/profiles/IniVisualEditor.tsx` — form fields grouped by category
- [ ] **1.6.3** Create `src/components/profiles/IniRawEditor.tsx` — textarea for raw INI text
- [ ] **1.6.4** Create `src/components/profiles/IniSplitEditor.tsx` — side-by-side visual + raw
- [ ] **1.6.5** Mode toggle: Visual / Raw / Split tabs
- [ ] **1.6.6** Sync logic: edits in visual → update raw; edits in raw → update visual (debounced)
- [ ] **1.6.7** Save button: serialize to JSON + write INI files

## Phase 1.7: Validation & Error Handling

- [ ] **1.7.1** Add `src/lib/validation.ts` with Zod schemas for Profile fields
- [ ] **1.7.2** Port: integer 27000-27015, no conflicts with other profiles
- [ ] **1.7.3** Admin password: non-empty, min 4 chars
- [ ] **1.7.4** Map: enum of valid ARK maps (TheIsland, TheCenter, ScorchedEarth, etc.)
- [ ] **1.7.5** Inline error display: red border + error message below field
- [ ] **1.7.6** Block save: disable Save button while form is invalid
- [ ] **1.7.7** Error toast: user-facing error message on Tauri command failure

## Phase 1.8: Delete Confirmation Dialog

- [ ] **1.8.1** Create `src/components/ui/ConfirmDialog.tsx` reusable component
- [ ] **1.8.2** Triggered by delete quick-action on profile card
- [ ] **1.8.3** Shows: "Delete '{profileName}'? This cannot be undone."
- [ ] **1.8.4** Confirm button: red/destructive style; Cancel button: secondary
- [ ] **1.8.5** On confirm: calls `deleteProfile`, closes dialog, refreshes list

## Phase 1.9: Wiring & Integration

- [ ] **1.9.1** Add `ProfileWizard` to main App layout (conditionally rendered)
- [ ] **1.9.2** Route: clicking a profile card opens `ProfileEditor`
- [ ] **1.9.3** Ensure profile list refreshes after create/edit/delete
- [ ] **1.9.4** Test create → edit → delete flow manually
- [ ] **1.9.5** Verify `pnpm run check` passes (zero errors)
- [ ] **1.9.6** Verify `pnpm test` runs

## Phase 1.10: Finalize

- [ ] **1.10.1** Verify all acceptance criteria in `spec.md`
- [ ] **1.10.2** Commit with `feat(phase1): Implement server profile management`
- [ ] **1.10.3** Attach task summary as git note
- [ ] **1.10.4** Update this plan: mark all tasks `[x]` with commit SHAs

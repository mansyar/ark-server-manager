# Phase 1 — Server Profile Management: Implementation Plan

## Phase 1.1: Rust Backend — Profile Storage Tauri Commands

- [x] **1.1.1** Define `Profile` Rust struct with serde derive (name, map, difficulty, maxPlayers, adminPassword, port, extraSettings, schemaVersion)
- [x] **1.1.2** Add Tauri commands: `list_profiles`, `load_profile`, `save_profile`, `delete_profile`
- [x] **1.1.3** Implement `list_profiles` — reads `%APPDATA%\ArkServerManager\profiles\`, returns `Vec<ProfileMetadata>`
- [x] **1.1.4** Implement `save_profile` — serializes Profile to JSON, writes to `{name}.json`
- [x] **1.1.5** Implement `load_profile` — reads JSON, deserializes to Profile
- [x] **1.1.6** Implement `delete_profile` — removes `{name}.json` from disk
- [x] **1.1.7** Add error handling: missing directory → auto-create; file not found → `ProfileNotFound` error
- [x] **1.1.8** Run `cargo clippy --all-targets --all-features` — zero warnings
- [x] **1.1.9** Write unit tests for profile serde (round-trip: JSON → Profile → JSON)

## Phase 1.2: INI Parsing & Serialization

- [x] **1.2.1** Add `configparser` crate to `Cargo.toml` for INI parsing
- [x] **1.2.2** Implement `profile_to_game_ini`: convert Profile → Game.ini string
- [x] **1.2.3** Implement `profile_to_game_user_settings_ini`: Profile → GameUserSettings.ini string
- [x] **1.2.4** Handle ARK-specific INI sections: `[ServerSettings]`, `[GameUserSettings]`
- [x] **1.2.5** Add unit tests for INI ↔ Profile round-trip
- [x] **1.2.6** Run `cargo clippy --all-targets --all-features` — zero warnings

## Phase 1.3: Zustand Store — Profiles Slice

- [x] **1.3.1** Create `src/stores/profilesStore.ts` with Zustand slice
- [x] **1.3.2** Define store shape: `profiles: Profile[]`, `activeProfile: Profile | null`, `isLoading: boolean`, `error: string | null`
- [x] **1.3.3** Add actions: `loadProfiles`, `createProfile`, `updateProfile`, `deleteProfile`, `setActiveProfile`
- [x] **1.3.4** Integrate with Tauri invoke calls (async, non-blocking)
- [x] **1.3.5** Add error handling in store actions (set `error` state on failure)
- [x] **1.3.6** Export typed hooks: `useProfilesStore`, `useActiveProfile`

## Phase 1.4: UI — Profile List View

- [x] **1.4.1** Create `src/components/profiles/ProfileListView.tsx`
- [x] **1.4.2** Display profile cards: name, map, status indicator, last modified date
- [x] **1.4.3** Add "Create New Profile" button (opens wizard)
- [x] **1.4.4** Add edit/delete quick-action buttons on each card
- [x] **1.4.5** Empty state: large "Create Your First Server" CTA with illustration
- [x] **1.4.6** Connect to `useProfilesStore` — load profiles on mount

## Phase 1.5: UI — Profile Creation Wizard

- [x] **1.5.1** Create `src/components/profiles/ProfileWizard.tsx` as multi-step modal/drawer
- [x] **1.5.2** Step 1: Profile name input with validation (unique, non-empty, filename-safe)
- [x] **1.5.3** Step 2: Server settings (map selector, difficulty slider 0-20, max players 1-100)
- [x] **1.5.4** Step 3: Admin password + confirm, port number (27000-27015 range)
- [x] **1.5.5** Step 4: Review summary before saving
- [x] **1.5.6** Progress indicator showing current step
- [x] **1.5.7** Back/Next navigation; "Create Profile" button on final step
- [x] **1.5.8** On submit: call `createProfile` Tauri command, close wizard, refresh list
- [x] **1.5.9** Validation: inline errors per field, red border, block Next until valid

## Phase 1.6: UI — INI Editor (Split View)

- [x] **1.6.1** Create `src/components/profiles/ProfileEditor.tsx` (combined visual/raw/split tabs)
- [x] **1.6.2** Visual mode: form fields grouped by category
- [x] **1.6.3** Raw mode: textarea with JSON
- [x] **1.6.4** Split mode: side-by-side visual + raw
- [x] **1.6.5** Mode toggle: Visual / Raw / Split tabs
- [x] **1.6.6** Sync logic: edits in visual → update raw; edits in raw → update visual (debounced)
- [x] **1.6.7** Save button: calls `updateProfile` on Tauri backend

## Phase 1.7: Validation & Error Handling

- [x] **1.7.1** Add `src/lib/validation.ts` with Zod schemas for Profile fields
- [x] **1.7.2** Port: integer 27000-27015, no conflicts with other profiles
- [x] **1.7.3** Admin password: non-empty, min 4 chars
- [x] **1.7.4** Map: enum of valid ARK maps (TheIsland, TheCenter, ScorchedEarth, etc.)
- [x] **1.7.5** Inline error display: red border + error message below field
- [x] **1.7.6** Block save: disable Save button while form is invalid
- [x] **1.7.7** Error toast: user-facing error message on Tauri command failure

## Phase 1.8: Delete Confirmation Dialog

- [x] **1.8.1** Create `src/components/ui/ConfirmDialog.tsx` reusable component
- [x] **1.8.2** Triggered by delete quick-action on profile card
- [x] **1.8.3** Shows: "Delete '{profileName}'? This cannot be undone."
- [x] **1.8.4** Confirm button: red/destructive style; Cancel button: secondary
- [x] **1.8.5** On confirm: calls `deleteProfile`, closes dialog, refreshes list

## Phase 1.9: Wiring & Integration

- [x] **1.9.1** Add `ProfileWizard` to main App layout (conditionally rendered)
- [x] **1.9.2** Route: clicking a profile card opens `ProfileEditor`
- [x] **1.9.3** Ensure profile list refreshes after create/edit/delete
- [x] **1.9.4** Test create → edit → delete flow manually
- [x] **1.9.5** Verify `pnpm run check` passes (zero errors)
- [x] **1.9.6** Verify `pnpm test` runs

## Phase 1.10: Finalize

- [x] **1.10.1** Verify all acceptance criteria in `spec.md`
- [x] **1.10.2** Commit with `feat(phase1): Implement server profile management`
- [x] **1.10.3** Attach task summary as git note
- [x] **1.10.4** Update this plan: mark all tasks `[x]` with commit SHAs

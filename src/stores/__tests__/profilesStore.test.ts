import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useProfilesStore } from '../profilesStore';
import { invoke } from '@tauri-apps/api/core';

// Mock Tauri API - must be set up before using vi.mocked
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('profilesStore', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store state
    useProfilesStore.setState({
      profiles: [],
      activeProfile: null,
      isLoading: false,
      error: null,
      wizardOpen: false,
      editorOpen: false,
    });
  });

  it('has correct initial state', () => {
    const state = useProfilesStore.getState();
    expect(state.profiles).toEqual([]);
    expect(state.activeProfile).toBeNull();
    expect(state.isLoading).toBe(false);
    expect(state.error).toBeNull();
    expect(state.wizardOpen).toBe(false);
    expect(state.editorOpen).toBe(false);
  });

  describe('loadProfiles', () => {
    it('loads profiles successfully', async () => {
      vi.mocked(invoke).mockResolvedValue([
        { name: 'Server1', map: 'TheIsland', last_modified: '2025-04-08T00:00:00Z' },
      ]);

      await useProfilesStore.getState().loadProfiles();

      const state = useProfilesStore.getState();
      expect(state.profiles).toHaveLength(1);
      expect(state.profiles[0].name).toBe('Server1');
      expect(state.isLoading).toBe(false);
    });

    it('handles load error', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Failed to load'));

      await useProfilesStore.getState().loadProfiles();

      const state = useProfilesStore.getState();
      expect(state.error).toBe('Error: Failed to load');
      expect(state.isLoading).toBe(false);
    });
  });

  describe('createProfile', () => {
    it('creates profile successfully', async () => {
      // Mock: first call (save_profile) returns undefined, second call (loadProfiles) returns empty array
      vi.mocked(invoke).mockResolvedValueOnce(undefined).mockResolvedValueOnce([]);

      const newProfile = {
        schema_version: 1,
        name: 'NewServer',
        map: 'TheIsland',
        difficulty: 1.0,
        max_players: 70,
        admin_password: null,
        port: 27015,
        server_install_path: null,
        steamcmd_path: null,
        extra_settings: {},
        extra_user_settings: {},
      };

      await useProfilesStore.getState().createProfile(newProfile);

      const state = useProfilesStore.getState();
      expect(state.wizardOpen).toBe(false);
    });
  });

  describe('setActiveProfile', () => {
    it('sets active profile', () => {
      const mockProfile = {
        schema_version: 1,
        name: 'TestServer',
        map: 'TheIsland',
        difficulty: 1.0,
        max_players: 70,
        admin_password: null,
        port: 27015,
        server_install_path: null,
        steamcmd_path: null,
        extra_settings: {},
        extra_user_settings: {},
      };
      useProfilesStore.getState().setActiveProfile(mockProfile);
      expect(useProfilesStore.getState().activeProfile).toEqual(mockProfile);
    });

    it('clears active profile', () => {
      const mockProfile = {
        schema_version: 1,
        name: 'TestServer',
        map: 'TheIsland',
        difficulty: 1.0,
        max_players: 70,
        admin_password: null,
        port: 27015,
        server_install_path: null,
        steamcmd_path: null,
        extra_settings: {},
        extra_user_settings: {},
      };
      useProfilesStore.setState({ activeProfile: mockProfile });
      useProfilesStore.getState().setActiveProfile(null);
      expect(useProfilesStore.getState().activeProfile).toBeNull();
    });
  });

  describe('setWizardOpen', () => {
    it('opens wizard', () => {
      useProfilesStore.getState().setWizardOpen(true);
      expect(useProfilesStore.getState().wizardOpen).toBe(true);
    });

    it('closes wizard', () => {
      useProfilesStore.setState({ wizardOpen: true });
      useProfilesStore.getState().setWizardOpen(false);
      expect(useProfilesStore.getState().wizardOpen).toBe(false);
    });
  });

  describe('setEditorOpen', () => {
    it('opens editor', () => {
      useProfilesStore.getState().setEditorOpen(true);
      expect(useProfilesStore.getState().editorOpen).toBe(true);
    });
  });

  describe('updateProfile', () => {
    it('updates profile successfully', async () => {
      vi.mocked(invoke).mockResolvedValueOnce(undefined).mockResolvedValueOnce([]);

      const updatedProfile = {
        schema_version: 1,
        name: 'TestServer',
        map: 'TheIsland',
        difficulty: 1.0,
        max_players: 70,
        admin_password: null,
        port: 27015,
        server_install_path: null,
        steamcmd_path: null,
        extra_settings: {},
        extra_user_settings: {},
      };

      await useProfilesStore.getState().updateProfile(updatedProfile);

      const state = useProfilesStore.getState();
      expect(state.activeProfile).toEqual(updatedProfile);
    });

    it('handles update error', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Update failed'));

      const profile = {
        schema_version: 1,
        name: 'TestServer',
        map: 'TheIsland',
        difficulty: 1.0,
        max_players: 70,
        admin_password: null,
        port: 27015,
        server_install_path: null,
        steamcmd_path: null,
        extra_settings: {},
        extra_user_settings: {},
      };

      await useProfilesStore.getState().updateProfile(profile);

      const state = useProfilesStore.getState();
      expect(state.error).toBe('Error: Update failed');
      expect(state.isLoading).toBe(false);
    });
  });

  describe('deleteProfile', () => {
    it('deletes profile successfully', async () => {
      vi.mocked(invoke).mockResolvedValueOnce(undefined).mockResolvedValueOnce([]);

      await useProfilesStore.getState().deleteProfile('TestServer');

      const state = useProfilesStore.getState();
      expect(state.activeProfile).toBeNull();
      expect(state.editorOpen).toBe(false);
    });

    it('handles delete error', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Delete failed'));

      await useProfilesStore.getState().deleteProfile('TestServer');

      const state = useProfilesStore.getState();
      expect(state.error).toBe('Error: Delete failed');
      expect(state.isLoading).toBe(false);
    });
  });
});

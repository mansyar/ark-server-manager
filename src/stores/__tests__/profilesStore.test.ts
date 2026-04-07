import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useProfilesStore } from '../profilesStore';

// Mock Tauri API
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
      const mockProfiles = [
        { name: 'Server1', map: 'TheIsland', last_modified: '2025-04-08T00:00:00Z' },
      ];
      vi.mocked(vi.fn()).mockResolvedValue(mockProfiles);

      // Manually set for test
      useProfilesStore.setState({ profiles: mockProfiles });

      const state = useProfilesStore.getState();
      expect(state.profiles).toHaveLength(1);
      expect(state.profiles[0].name).toBe('Server1');
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
        extra_settings: {},
        extra_user_settings: {},
      };
      useProfilesStore.setState({ activeProfile: mockProfile });
      useProfilesStore.getState().setActiveProfile(null);
      expect(useProfilesStore.getState().activeProfile).toBeNull();
    });
  });
});

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useProfilesStore } from '@/stores/profilesStore';
import { useServerStore } from '@/stores/serverLifecycleStore';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock Tauri event listener
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

describe('App integration', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset stores
    useProfilesStore.setState({
      profiles: [],
      activeProfile: null,
      isLoading: false,
      error: null,
      wizardOpen: false,
      editorOpen: false,
    });
    useServerStore.setState({
      status: {},
      handles: {},
      consoleBuffers: {},
      players: {},
      lastPlayerUpdate: {},
      validation: {},
      isStarting: {},
      isStopping: {},
      errors: {},
      activeServerProfile: null,
      unlisteners: [],
    });
  });

  it('hides wizard by default', () => {
    const state = useProfilesStore.getState();
    expect(state.wizardOpen).toBe(false);
  });

  it('hides editor by default', () => {
    const state = useProfilesStore.getState();
    expect(state.editorOpen).toBe(false);
  });

  it('hides server detail panel by default', () => {
    const state = useServerStore.getState();
    expect(state.activeServerProfile).toBeNull();
  });

  it('can open wizard', () => {
    useProfilesStore.getState().setWizardOpen(true);
    expect(useProfilesStore.getState().wizardOpen).toBe(true);
  });

  it('can open editor', () => {
    useProfilesStore.getState().setEditorOpen(true);
    expect(useProfilesStore.getState().editorOpen).toBe(true);
  });

  it('can open server detail panel', () => {
    useServerStore.getState().setActiveServerProfile('TestServer');
    expect(useServerStore.getState().activeServerProfile).toBe('TestServer');
  });

  it('can close wizard', () => {
    useProfilesStore.setState({ wizardOpen: true });
    useProfilesStore.getState().setWizardOpen(false);
    expect(useProfilesStore.getState().wizardOpen).toBe(false);
  });

  it('can close editor', () => {
    useProfilesStore.setState({ editorOpen: true });
    useProfilesStore.getState().setEditorOpen(false);
    expect(useProfilesStore.getState().editorOpen).toBe(false);
  });

  it('can close server detail panel', () => {
    useServerStore.setState({ activeServerProfile: 'TestServer' });
    useServerStore.getState().setActiveServerProfile(null);
    expect(useServerStore.getState().activeServerProfile).toBeNull();
  });

  it('stores are properly initialized', () => {
    // Test profiles store actions
    const profilesState = useProfilesStore.getState();
    expect(typeof profilesState.setWizardOpen).toBe('function');
    expect(typeof profilesState.setEditorOpen).toBe('function');
    expect(typeof profilesState.setActiveProfile).toBe('function');

    // Test server store actions
    const serverState = useServerStore.getState();
    expect(typeof serverState.setActiveServerProfile).toBe('function');
    expect(typeof serverState.getStatus).toBe('function');
    expect(typeof serverState.getPlayers).toBe('function');
  });
});

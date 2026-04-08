import { vi, beforeEach } from 'vitest';
import { useServerStore } from '../serverLifecycleStore';

// Shared beforeEach that resets store state (mocks must be set up in each test file)
export function setupStoreForTesting() {
  beforeEach(() => {
    vi.clearAllMocks();
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
}

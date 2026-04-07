import { describe, it, expect } from 'vitest';
import { useAppStore } from '../appStore';

describe('appStore', () => {
  it('creates store with empty state', () => {
    const state = useAppStore.getState();
    expect(state).toEqual({});
  });

  it('allows extending state', () => {
    // AppStore is defined as object with no specific state
    // This test validates the store creation works
    expect(useAppStore).toBeDefined();
    expect(typeof useAppStore.getState).toBe('function');
    expect(typeof useAppStore.setState).toBe('function');
    expect(typeof useAppStore.subscribe).toBe('function');
  });
});

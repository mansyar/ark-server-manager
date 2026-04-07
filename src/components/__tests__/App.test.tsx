import { describe, it, expect, vi } from 'vitest';

// Mock Tauri API before importing components
vi.mock('@tauri-apps/api', () => ({
  invoke: vi.fn(),
}));

describe('App', () => {
  it('placeholder test - infrastructure validation', () => {
    expect(true).toBe(true);
  });
});

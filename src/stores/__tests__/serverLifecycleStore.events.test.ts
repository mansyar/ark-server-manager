import { describe, it, expect, vi } from 'vitest';
import { useServerStore } from '../serverLifecycleStore';
import { listen } from '@tauri-apps/api/event';
import { setupStoreForTesting } from './serverLifecycleStore.setup';
import type { ServerStatus, PlayerInfo } from '@/types/server';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

describe('serverLifecycleStore - events', () => {
  setupStoreForTesting();

  describe('event handler callbacks', () => {
    it('handles server-started event callback', async () => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      vi.mocked(listen).mockImplementation((eventName: string, handler: (event: any) => void) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      const startedHandler = callbacks.find((c) => c.eventName === 'server-started');
      expect(startedHandler).toBeDefined();

      // Trigger the handler
      startedHandler!.handler({
        payload: {
          profile_name: 'TestServer',
          pid: 1234,
          port: 27015,
          started_at: '2025-04-08T00:00:00Z',
        },
      });

      expect(useServerStore.getState().status.TestServer).toBe('Running');
      expect(useServerStore.getState().handles.TestServer).toBeDefined();
    });

    it('handles server-stopped event callback', async () => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      vi.mocked(listen).mockImplementation((eventName: string, handler: (event: any) => void) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      const stoppedHandler = callbacks.find((c) => c.eventName === 'server-stopped');
      expect(stoppedHandler).toBeDefined();

      // Set up state first
      useServerStore.setState({
        status: { TestServer: 'Running' as ServerStatus },
        handles: {
          TestServer: {
            pid: 1234,
            profile_name: 'TestServer',
            started_at: '2025-04-08T00:00:00Z',
            ark_exe_path: '/path',
            port: 27015,
          },
        },
        isStopping: { TestServer: true },
      });

      // Trigger the handler
      stoppedHandler!.handler({ payload: { profile_name: 'TestServer' } });

      expect(useServerStore.getState().status.TestServer).toBe('Stopped');
      expect(useServerStore.getState().isStopping.TestServer).toBe(false);
    });

    it('handles status-changed event callback', async () => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      vi.mocked(listen).mockImplementation((eventName: string, handler: (event: any) => void) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      const statusHandler = callbacks.find((c) => c.eventName === 'status-changed');
      expect(statusHandler).toBeDefined();

      // Trigger the handler with Crashed status
      statusHandler!.handler({
        payload: { profile_name: 'TestServer', status: 'Crashed' },
      });

      expect(useServerStore.getState().status.TestServer).toBe('Crashed');
    });

    it('handles server-crashed event callback', async () => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      vi.mocked(listen).mockImplementation((eventName: string, handler: (event: any) => void) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      const crashedHandler = callbacks.find((c) => c.eventName === 'server-crashed');
      expect(crashedHandler).toBeDefined();

      // Trigger the handler with the correct payload format
      crashedHandler!.handler({
        payload: {
          profile_name: 'TestServer',
          error: 'Segmentation fault',
        },
      });

      expect(useServerStore.getState().status.TestServer).toBe('Crashed');
      expect(useServerStore.getState().errors.TestServer).toContain('Segmentation fault');
    });

    it('handles console-output event callback', async () => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      vi.mocked(listen).mockImplementation((eventName: string, handler: (event: any) => void) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      const consoleHandler = callbacks.find((c) => c.eventName === 'console-output');
      expect(consoleHandler).toBeDefined();

      // Trigger the handler
      consoleHandler!.handler({
        payload: {
          profile_name: 'TestServer',
          line: 'Server console output',
          source: 'stdout',
          timestamp: '2025-04-08T00:00:00Z',
        },
      });

      const buffer = useServerStore.getState().consoleBuffers.TestServer;
      expect(buffer).toHaveLength(1);
      expect(buffer[0].line).toBe('Server console output');
    });

    it('handles player-list-updated event callback', async () => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      vi.mocked(listen).mockImplementation((eventName: string, handler: (event: any) => void) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      const playerHandler = callbacks.find((c) => c.eventName === 'player-list-updated');
      expect(playerHandler).toBeDefined();

      const mockPlayers: PlayerInfo[] = [
        {
          player_name: 'Player1',
          player_id: '123',
          tribe: 'Tribe1',
          join_time: '2025-04-08T00:00:00Z',
        },
        {
          player_name: 'Player2',
          player_id: '456',
          tribe: null,
          join_time: '2025-04-08T00:01:00Z',
        },
      ];

      // Trigger the handler
      playerHandler!.handler({
        payload: { profile_name: 'TestServer', players: mockPlayers },
      });

      const players = useServerStore.getState().players.TestServer;
      expect(players).toHaveLength(2);
      expect(players[0].player_name).toBe('Player1');
      expect(useServerStore.getState().lastPlayerUpdate.TestServer).toBeDefined();
    });
  });
});

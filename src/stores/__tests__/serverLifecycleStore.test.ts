import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useServerStore } from '../serverLifecycleStore';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { ServerStatus, ConsoleLine, PlayerInfo } from '@/types/server';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock Tauri event listener
vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

describe('serverLifecycleStore', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store state
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

  describe('initial state', () => {
    it('has correct initial state', () => {
      const state = useServerStore.getState();
      expect(state.status).toEqual({});
      expect(state.handles).toEqual({});
      expect(state.consoleBuffers).toEqual({});
      expect(state.players).toEqual({});
      expect(state.errors).toEqual({});
      expect(state.activeServerProfile).toBeNull();
    });
  });

  describe('getStatus', () => {
    it('returns Stopped for unknown profile', () => {
      const status = useServerStore.getState().getStatus('unknown');
      expect(status).toBe('Stopped');
    });

    it('returns correct status for known profile', () => {
      useServerStore.setState({
        status: { TestServer: 'Running' as ServerStatus },
      });
      const status = useServerStore.getState().getStatus('TestServer');
      expect(status).toBe('Running');
    });
  });

  describe('clearConsoleBuffer', () => {
    it('clears console buffer for profile', () => {
      useServerStore.setState({
        consoleBuffers: {
          TestServer: [
            {
              profile_name: 'TestServer',
              timestamp: '2025-04-08T00:00:00Z',
              line: 'test',
              source: 'stdout' as const,
            },
          ],
        },
      });
      useServerStore.getState().clearConsoleBuffer('TestServer');
      expect(useServerStore.getState().consoleBuffers.TestServer).toEqual([]);
    });
  });

  describe('addConsoleLine', () => {
    it('adds console line to buffer', () => {
      const line: ConsoleLine = {
        profile_name: 'TestServer',
        timestamp: '2025-04-08T00:00:00Z',
        line: 'Server started',
        source: 'stdout',
      };
      useServerStore.getState().addConsoleLine(line);
      const buffer = useServerStore.getState().consoleBuffers.TestServer;
      expect(buffer).toHaveLength(1);
      expect(buffer[0].line).toBe('Server started');
    });

    it('limits buffer to 1000 lines', () => {
      // Set up a buffer with 1000 lines
      const lines: ConsoleLine[] = [];
      for (let i = 0; i < 1001; i++) {
        lines.push({
          profile_name: 'TestServer',
          timestamp: `2025-04-08T00:00:${i.toString().padStart(2, '0')}Z`,
          line: `Line ${i}`,
          source: 'stdout',
        });
      }
      // Set state with 1000 existing lines
      useServerStore.setState({
        consoleBuffers: { TestServer: lines.slice(0, 1000) },
      });

      // Add one more
      useServerStore.getState().addConsoleLine({
        profile_name: 'TestServer',
        timestamp: '2025-04-08T00:00:00Z',
        line: 'Line 1001',
        source: 'stdout',
      });

      const buffer = useServerStore.getState().consoleBuffers.TestServer;
      expect(buffer).toHaveLength(1000);
    });
  });

  describe('getPlayers', () => {
    it('returns empty array for unknown profile', () => {
      const players = useServerStore.getState().getPlayers('unknown');
      expect(players).toEqual([]);
    });

    it('returns players for known profile', () => {
      const mockPlayers: PlayerInfo[] = [
        {
          player_name: 'Player1',
          player_id: '123',
          tribe: 'Tribe1',
          join_time: '2025-04-08T00:00:00Z',
        },
      ];
      useServerStore.setState({
        players: { TestServer: mockPlayers },
      });
      const players = useServerStore.getState().getPlayers('TestServer');
      expect(players).toHaveLength(1);
      expect(players[0].player_name).toBe('Player1');
    });
  });

  describe('setActiveServerProfile', () => {
    it('sets active server profile', () => {
      useServerStore.getState().setActiveServerProfile('TestServer');
      expect(useServerStore.getState().activeServerProfile).toBe('TestServer');
    });

    it('clears active server profile', () => {
      useServerStore.setState({ activeServerProfile: 'TestServer' });
      useServerStore.getState().setActiveServerProfile(null);
      expect(useServerStore.getState().activeServerProfile).toBeNull();
    });
  });

  describe('cleanupListeners', () => {
    it('cleans up listeners', () => {
      const mockUnlisten = vi.fn();
      useServerStore.setState({
        unlisteners: [mockUnlisten, mockUnlisten],
      });
      useServerStore.getState().cleanupListeners();
      expect(mockUnlisten).toHaveBeenCalledTimes(2);
      expect(useServerStore.getState().unlisteners).toHaveLength(0);
    });
  });

  describe('refreshStatus', () => {
    it('refreshes status from backend', async () => {
      vi.mocked(invoke).mockResolvedValue('Running');

      await useServerStore.getState().refreshStatus('TestServer');

      expect(invoke).toHaveBeenCalledWith('get_server_status', { profileName: 'TestServer' });
      expect(useServerStore.getState().status.TestServer).toBe('Running');
    });

    it('handles refresh error', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Backend error'));
      const consoleSpy = vi.spyOn(console, 'error').mockReturnValue();

      await useServerStore.getState().refreshStatus('TestServer');

      expect(consoleSpy).toHaveBeenCalledWith('Failed to refresh server status:', expect.any(Error));
      consoleSpy.mockRestore();
    });
  });

  describe('startServer', () => {
    it('starts server successfully', async () => {
      vi.mocked(invoke).mockResolvedValue({
        pid: 1234,
        profile_name: 'TestServer',
        started_at: '2025-04-08T00:00:00Z',
        ark_exe_path: '/path/to/ark',
        port: 27015,
      });

      await useServerStore.getState().startServer('TestServer');

      expect(useServerStore.getState().isStarting.TestServer).toBe(true);
      expect(useServerStore.getState().status.TestServer).toBe('Starting');
      expect(invoke).toHaveBeenCalledWith('start_server', { profileName: 'TestServer' });
    });

    it('handles start error', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Start failed'));

      await useServerStore.getState().startServer('TestServer');

      expect(useServerStore.getState().status.TestServer).toBe('Stopped');
      expect(useServerStore.getState().isStarting.TestServer).toBe(false);
      expect(useServerStore.getState().errors.TestServer).toBe('Error: Start failed');
    });
  });

  describe('stopServer', () => {
    it('stops server successfully', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);

      await useServerStore.getState().stopServer('TestServer');

      expect(useServerStore.getState().isStopping.TestServer).toBe(true);
      expect(invoke).toHaveBeenCalledWith('stop_server', { profileName: 'TestServer' });
    });

    it('handles stop error', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Stop failed'));

      await useServerStore.getState().stopServer('TestServer');

      expect(useServerStore.getState().isStopping.TestServer).toBe(false);
      expect(useServerStore.getState().errors.TestServer).toBe('Error: Stop failed');
    });
  });

  describe('restartServer', () => {
    it('restarts server', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);

      useServerStore.setState({
        status: { TestServer: 'Running' },
        isStopping: { TestServer: false },
      });

      await useServerStore.getState().restartServer('TestServer');

      // Should have called stopServer then startServer
      expect(invoke).toHaveBeenCalledWith('stop_server', { profileName: 'TestServer' });
    });
  });

  describe('validateInstall', () => {
    it('validates install successfully', async () => {
      const mockResult = {
        is_valid: true,
        message: 'OK',
        ark_exe_path: '/path/to/ark',
      };
      vi.mocked(invoke).mockResolvedValue(mockResult);

      const result = await useServerStore.getState().validateInstall('TestServer');

      expect(result).toEqual(mockResult);
      expect(useServerStore.getState().validation.TestServer).toEqual(mockResult);
    });

    it('handles validation error', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Validation failed'));

      const result = await useServerStore.getState().validateInstall('TestServer');

      expect(result.is_valid).toBe(false);
      expect(result.message).toBe('Error: Validation failed');
      expect(useServerStore.getState().validation.TestServer).toEqual({
        is_valid: false,
        message: 'Error: Validation failed',
        ark_exe_path: null,
      });
    });
  });

  describe('getConsoleBuffer', () => {
    it('fetches console buffer from backend', async () => {
      const mockBuffer = [
        { profile_name: 'TestServer', timestamp: '2025-04-08T00:00:00Z', line: 'test', source: 'stdout' },
      ];
      vi.mocked(invoke).mockResolvedValue(mockBuffer);

      const buffer = await useServerStore.getState().getConsoleBuffer('TestServer');

      expect(buffer).toEqual(mockBuffer);
      expect(useServerStore.getState().consoleBuffers.TestServer).toEqual(mockBuffer);
    });

    it('handles getConsoleBuffer error', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Failed to get buffer'));
      const consoleSpy = vi.spyOn(console, 'error').mockReturnValue();

      const result = await useServerStore.getState().getConsoleBuffer('TestServer');

      // Should return existing buffer (empty) on error
      expect(result).toEqual([]);
      expect(consoleSpy).toHaveBeenCalledWith('Failed to get console buffer:', expect.any(Error));
      consoleSpy.mockRestore();
    });
  });

  describe('initListeners', () => {
    it('sets up event listeners', async () => {
      vi.mocked(listen).mockResolvedValue(vi.fn());

      await useServerStore.getState().initListeners();

      // Should have registered multiple listeners
      expect(listen).toHaveBeenCalled();
      expect(useServerStore.getState().unlisteners).toHaveLength(6);
    });
  });
});

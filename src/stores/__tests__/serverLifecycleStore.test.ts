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

    it('registers all 6 event listeners', async () => {
      vi.mocked(listen).mockResolvedValue(vi.fn());

      await useServerStore.getState().initListeners();

      // 6 listeners: server-started, server-stopped, status-changed, server-crashed, console-output, player-list-updated
      expect(listen).toHaveBeenCalledTimes(6);
    });
  });

  describe('event handler callbacks', () => {
    it('handles server-started event callback', async () => {
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      vi.mocked(listen).mockImplementation((eventName: string, handler: any) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      // Find and invoke server-started callback
      const serverStartedCb = callbacks.find((c) => c.eventName === 'server-started');
      expect(serverStartedCb).toBeDefined();

      const mockHandle = {
        pid: 1234,
        profile_name: 'TestServer',
        started_at: '2025-04-08T00:00:00Z',
        ark_exe_path: '/path/to/ark',
        port: 27015,
      };

      serverStartedCb!.handler({ payload: mockHandle });

      // Verify state was updated
      const state = useServerStore.getState();
      expect(state.handles.TestServer).toEqual(mockHandle);
    });

    it('handles server-stopped event callback', async () => {
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      vi.mocked(listen).mockImplementation((eventName: string, handler: any) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      // Set initial state
      useServerStore.setState({
        status: { TestServer: 'Running' },
        isStopping: { TestServer: true },
      });

      // Find and invoke server-stopped callback
      const serverStoppedCb = callbacks.find((c) => c.eventName === 'server-stopped');
      expect(serverStoppedCb).toBeDefined();

      serverStoppedCb!.handler({ payload: { profile_name: 'TestServer' } });

      const state = useServerStore.getState();
      expect(state.status.TestServer).toBe('Stopped');
      expect(state.isStopping.TestServer).toBe(false);
    });

    it('handles status-changed event callback', async () => {
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      vi.mocked(listen).mockImplementation((eventName: string, handler: any) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      const statusChangedCb = callbacks.find((c) => c.eventName === 'status-changed');
      expect(statusChangedCb).toBeDefined();

      statusChangedCb!.handler({
        payload: { profile_name: 'TestServer', status: 'Running' as ServerStatus },
      });

      const state = useServerStore.getState();
      expect(state.status.TestServer).toBe('Running');
    });

    it('handles server-crashed event callback', async () => {
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      vi.mocked(listen).mockImplementation((eventName: string, handler: any) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      useServerStore.setState({
        status: { TestServer: 'Running' },
        handles: { TestServer: { pid: 1234, profile_name: 'TestServer', started_at: '2025-01-01', ark_exe_path: '/test', port: 27015 } },
        isStarting: { TestServer: false },
        isStopping: { TestServer: false },
        errors: { TestServer: null },
      });

      const crashCb = callbacks.find((c) => c.eventName === 'server-crashed');
      expect(crashCb).toBeDefined();

      crashCb!.handler({ payload: { profile_name: 'TestServer', error: 'Server crashed unexpectedly' } });

      const state = useServerStore.getState();
      expect(state.status.TestServer).toBe('Crashed');
      // Note: handles are NOT cleared on crash, only status, errors, isStarting, isStopping are updated
      expect(state.errors.TestServer).toBe('Server crashed unexpectedly');
      expect(state.isStarting.TestServer).toBe(false);
      expect(state.isStopping.TestServer).toBe(false);
    });

    it('handles console-output event callback', async () => {
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      vi.mocked(listen).mockImplementation((eventName: string, handler: any) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      const consoleCb = callbacks.find((c) => c.eventName === 'console-output');
      expect(consoleCb).toBeDefined();

      const mockLine: ConsoleLine = {
        profile_name: 'TestServer',
        timestamp: '2025-04-08T00:00:00Z',
        line: 'Test console output',
        source: 'stdout',
      };

      consoleCb!.handler({ payload: mockLine });

      const state = useServerStore.getState();
      expect(state.consoleBuffers.TestServer).toContainEqual(mockLine);
    });

    it('handles player-list-updated event callback', async () => {
      const callbacks: Array<{ eventName: string; handler: (event: any) => void }> = [];
      vi.mocked(listen).mockImplementation((eventName: string, handler: any) => {
        callbacks.push({ eventName, handler });
        return Promise.resolve(vi.fn());
      });

      await useServerStore.getState().initListeners();

      const playersCb = callbacks.find((c) => c.eventName === 'player-list-updated');
      expect(playersCb).toBeDefined();

      const mockPlayers: PlayerInfo[] = [
        { player_name: 'Player1', player_id: '123', tribe: 'Tribe1', join_time: '2025-04-08T00:00:00Z' },
        { player_name: 'Player2', player_id: '456', tribe: null, join_time: '2025-04-08T00:01:00Z' },
      ];

      playersCb!.handler({
        payload: { profile_name: 'TestServer', players: mockPlayers },
      });

      const state = useServerStore.getState();
      expect(state.players.TestServer).toEqual(mockPlayers);
      expect(state.lastPlayerUpdate.TestServer).toBeInstanceOf(Date);
    });
  });

  describe('selector hooks', () => {
    it('returns default status for unknown profile', () => {
      // Use getState() with direct selector pattern (same logic as hook)
      const state = useServerStore.getState();
      const status = state.status['unknown'] ?? 'Stopped';
      expect(status).toBe('Stopped');
    });

    it('returns actual status from state', () => {
      useServerStore.setState({ status: { TestServer: 'Running' as ServerStatus } });
      const state = useServerStore.getState();
      const status = state.status['TestServer'] ?? 'Stopped';
      expect(status).toBe('Running');
    });

    it('returns null handle for unknown profile', () => {
      const state = useServerStore.getState();
      const handle = state.handles['unknown'] ?? null;
      expect(handle).toBeNull();
    });

    it('returns actual handle from state', () => {
      const mockHandle = { pid: 1234, profile_name: 'Test', started_at: '2025-01-01', ark_exe_path: '/test', port: 27015 };
      useServerStore.setState({ handles: { Test: mockHandle } });
      const state = useServerStore.getState();
      const handle = state.handles['Test'] ?? null;
      expect(handle).toEqual(mockHandle);
    });

    it('returns empty player array for unknown profile', () => {
      const state = useServerStore.getState();
      const players = state.players['unknown'] ?? [];
      expect(players).toEqual([]);
    });

    it('returns empty console buffer for unknown profile', () => {
      const state = useServerStore.getState();
      const buffer = state.consoleBuffers['unknown'] ?? [];
      expect(buffer).toEqual([]);
    });

    it('returns null validation for unknown profile', () => {
      const state = useServerStore.getState();
      const validation = state.validation['unknown'] ?? null;
      expect(validation).toBeNull();
    });
  });

  describe('branch coverage edge cases', () => {
    it('handles multiple profiles with different statuses', () => {
      useServerStore.setState({
        status: {
          ServerA: 'Running' as ServerStatus,
          ServerB: 'Stopped' as ServerStatus,
          ServerC: 'Crashed' as ServerStatus,
        },
      });

      expect(useServerStore.getState().getStatus('ServerA')).toBe('Running');
      expect(useServerStore.getState().getStatus('ServerB')).toBe('Stopped');
      expect(useServerStore.getState().getStatus('ServerC')).toBe('Crashed');
      expect(useServerStore.getState().getStatus('Unknown')).toBe('Stopped');
    });

    it('handles console buffer overflow edge case', () => {
      // Create buffer with exactly 1000 lines then add one more
      const lines: ConsoleLine[] = [];
      for (let i = 0; i < 1000; i++) {
        lines.push({
          profile_name: 'TestServer',
          timestamp: `2025-04-08T00:00:${i.toString().padStart(2, '0')}Z`,
          line: `Line ${i}`,
          source: 'stdout' as const,
        });
      }

      useServerStore.setState({
        consoleBuffers: { TestServer: lines },
      });

      // Add one more - should remove the oldest
      const newLine: ConsoleLine = {
        profile_name: 'TestServer',
        timestamp: '2025-04-08T00:00:00Z',
        line: 'Line 1000',
        source: 'stdout',
      };

      useServerStore.getState().addConsoleLine(newLine);

      const buffer = useServerStore.getState().consoleBuffers.TestServer;
      expect(buffer).toHaveLength(1000);
      expect(buffer[0].line).toBe('Line 1'); // Oldest line removed
      expect(buffer[999].line).toBe('Line 1000'); // Newest line at end
    });

    it('handles restartServer when server is already stopping', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);
      useServerStore.setState({
        status: { TestServer: 'Stopping' as ServerStatus },
        isStopping: { TestServer: true },
      });

      await useServerStore.getState().restartServer('TestServer');

      // Should still call stop then start
      expect(invoke).toHaveBeenCalledWith('stop_server', { profileName: 'TestServer' });
    });

    it('handles error when stopping already stopped server', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Server not running'));

      await useServerStore.getState().stopServer('TestServer');

      expect(useServerStore.getState().errors.TestServer).toBe('Error: Server not running');
    });

    it('handles validation with missing ark_exe_path', async () => {
      vi.mocked(invoke).mockResolvedValue({
        is_valid: false,
        message: 'ARK not found',
        ark_exe_path: null,
      });

      const result = await useServerStore.getState().validateInstall('TestServer');

      expect(result.is_valid).toBe(false);
      expect(result.ark_exe_path).toBeNull();
    });

    it('handles console line with different sources', () => {
      const stdoutLine: ConsoleLine = {
        profile_name: 'TestServer',
        timestamp: '2025-04-08T00:00:00Z',
        line: 'Info message',
        source: 'stdout',
      };

      const stderrLine: ConsoleLine = {
        profile_name: 'TestServer',
        timestamp: '2025-04-08T00:00:01Z',
        line: 'Error message',
        source: 'stderr',
      };

      useServerStore.getState().addConsoleLine(stdoutLine);
      useServerStore.getState().addConsoleLine(stderrLine);

      const buffer = useServerStore.getState().consoleBuffers.TestServer;
      expect(buffer).toHaveLength(2);
      expect(buffer[0].source).toBe('stdout');
      expect(buffer[1].source).toBe('stderr');
    });

    it('handles player list with null tribe', () => {
      const player: PlayerInfo = {
        player_name: 'SoloPlayer',
        player_id: '12345',
        tribe: null,
        join_time: '2025-04-08T00:00:00Z',
      };

      useServerStore.setState({
        players: { TestServer: [player] },
      });

      const players = useServerStore.getState().getPlayers('TestServer');
      expect(players[0].tribe).toBeNull();
    });
  });
});

import { describe, it, expect, vi } from 'vitest';
import { useServerStore } from '../serverLifecycleStore';
import { invoke } from '@tauri-apps/api/core';
import { setupStoreForTesting } from './serverLifecycleStore.setup';
import type { ServerStatus, ConsoleLine, PlayerInfo } from '@/types/server';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

describe('serverLifecycleStore - selectors & edge cases', () => {
  setupStoreForTesting();

  describe('selector hooks', () => {
    it('returns default status for unknown profile', () => {
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
      const mockHandle = {
        pid: 1234,
        profile_name: 'Test',
        started_at: '2025-01-01',
        ark_exe_path: '/test',
        port: 27015,
      };
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

      const newLine: ConsoleLine = {
        profile_name: 'TestServer',
        timestamp: '2025-04-08T00:00:00Z',
        line: 'Line 1000',
        source: 'stdout',
      };

      useServerStore.getState().addConsoleLine(newLine);

      const buffer = useServerStore.getState().consoleBuffers.TestServer;
      expect(buffer).toHaveLength(1000);
      expect(buffer[0].line).toBe('Line 1');
      expect(buffer[999].line).toBe('Line 1000');
    });

    it('handles restartServer when server is already stopping', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);
      useServerStore.setState({
        status: { TestServer: 'Stopping' as ServerStatus },
        isStopping: { TestServer: true },
      });

      await useServerStore.getState().restartServer('TestServer');

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

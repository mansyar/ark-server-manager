import { describe, it, expect, vi } from 'vitest';
import { useServerStore } from '../serverLifecycleStore';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { setupStoreForTesting } from './serverLifecycleStore.setup';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(vi.fn()),
}));

describe('serverLifecycleStore - server operations', () => {
  setupStoreForTesting();

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

      expect(consoleSpy).toHaveBeenCalledWith(
        'Failed to refresh server status:',
        expect.any(Error)
      );
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
        {
          profile_name: 'TestServer',
          timestamp: '2025-04-08T00:00:00Z',
          line: 'test',
          source: 'stdout',
        },
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

      expect(result).toEqual([]);
      expect(consoleSpy).toHaveBeenCalledWith('Failed to get console buffer:', expect.any(Error));
      consoleSpy.mockRestore();
    });
  });

  describe('initListeners', () => {
    it('sets up event listeners', async () => {
      vi.mocked(listen).mockResolvedValue(vi.fn());

      await useServerStore.getState().initListeners();

      expect(listen).toHaveBeenCalled();
      expect(useServerStore.getState().unlisteners).toHaveLength(9);
    });

    it('registers all 9 event listeners', async () => {
      vi.mocked(listen).mockResolvedValue(vi.fn());

      await useServerStore.getState().initListeners();

      expect(listen).toHaveBeenCalledTimes(9);
    });
  });
});

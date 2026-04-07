import { describe, it, expect } from 'vitest';
import type {
  ServerStatus,
  ServerHandle,
  ConsoleLine,
  PlayerInfo,
  ValidationResult,
} from '../server';

describe('server types', () => {
  describe('ServerStatus', () => {
    it('accepts valid status values', () => {
      const statuses: ServerStatus[] = ['Stopped', 'Starting', 'Running', 'Stopping', 'Crashed'];
      expect(statuses).toHaveLength(5);
    });
  });

  describe('ServerHandle', () => {
    it('has required fields', () => {
      const handle: ServerHandle = {
        pid: 12345,
        profile_name: 'TestServer',
        started_at: '2025-04-08T00:00:00Z',
        ark_exe_path: '/path/to/ShooterGameServer.exe',
        port: 27015,
      };
      expect(handle.pid).toBe(12345);
      expect(handle.profile_name).toBe('TestServer');
      expect(handle.port).toBe(27015);
    });
  });

  describe('ConsoleLine', () => {
    it('has required fields', () => {
      const line: ConsoleLine = {
        profile_name: 'TestServer',
        timestamp: '2025-04-08T00:00:00Z',
        line: 'Server started',
        source: 'stdout',
      };
      expect(line.profile_name).toBe('TestServer');
      expect(line.source).toBe('stdout');
    });

    it('accepts stderr source', () => {
      const line: ConsoleLine = {
        profile_name: 'TestServer',
        timestamp: '2025-04-08T00:00:00Z',
        line: 'Error occurred',
        source: 'stderr',
      };
      expect(line.source).toBe('stderr');
    });
  });

  describe('PlayerInfo', () => {
    it('has required fields', () => {
      const player: PlayerInfo = {
        player_name: 'Player1',
        player_id: '12345',
        tribe: 'MyTribe',
        join_time: '2025-04-08T00:00:00Z',
      };
      expect(player.player_name).toBe('Player1');
      expect(player.tribe).toBe('MyTribe');
    });

    it('allows null tribe', () => {
      const player: PlayerInfo = {
        player_name: 'SoloPlayer',
        player_id: '12345',
        tribe: null,
        join_time: '2025-04-08T00:00:00Z',
      };
      expect(player.tribe).toBeNull();
    });
  });

  describe('ValidationResult', () => {
    it('has required fields', () => {
      const result: ValidationResult = {
        is_valid: true,
        message: null,
        ark_exe_path: '/path/to/ShooterGameServer.exe',
      };
      expect(result.is_valid).toBe(true);
    });

    it('allows error message', () => {
      const result: ValidationResult = {
        is_valid: false,
        message: 'ARK executable not found',
        ark_exe_path: null,
      };
      expect(result.is_valid).toBe(false);
      expect(result.message).toBe('ARK executable not found');
    });
  });
});

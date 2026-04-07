import { describe, it, expect } from 'vitest';
import { statusConfig } from '../ServerControls';

describe('ServerControls', () => {
  describe('statusConfig', () => {
    it('has config for all server statuses', () => {
      expect(statusConfig).toHaveProperty('Stopped');
      expect(statusConfig).toHaveProperty('Starting');
      expect(statusConfig).toHaveProperty('Running');
      expect(statusConfig).toHaveProperty('Stopping');
      expect(statusConfig).toHaveProperty('Crashed');
    });

    it('Stopped has correct properties', () => {
      const config = statusConfig.Stopped;
      expect(config.color).toBe('text-red-500');
      expect(config.label).toBe('Stopped');
      expect(config.icon).toBe('🔴');
    });

    it('Starting has correct properties', () => {
      const config = statusConfig.Starting;
      expect(config.color).toBe('text-yellow-500');
      expect(config.label).toBe('Starting');
      expect(config.icon).toBe('🟡');
    });

    it('Running has correct properties', () => {
      const config = statusConfig.Running;
      expect(config.color).toBe('text-green-500');
      expect(config.label).toBe('Running');
      expect(config.icon).toBe('🟢');
    });

    it('Stopping has correct properties', () => {
      const config = statusConfig.Stopping;
      expect(config.color).toBe('text-orange-500');
      expect(config.label).toBe('Stopping');
      expect(config.icon).toBe('🟠');
    });

    it('Crashed has correct properties', () => {
      const config = statusConfig.Crashed;
      expect(config.color).toBe('text-destructive');
      expect(config.label).toBe('Crashed');
      expect(config.icon).toBe('⚠️');
    });
  });
});

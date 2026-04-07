import { describe, it, expect } from 'vitest';
import {
  profileNameSchema,
  difficultySchema,
  maxPlayersSchema,
  portSchema,
  createProfileSchema,
} from '../validation';

describe('validation', () => {
  describe('profileNameSchema', () => {
    it('accepts valid profile names', () => {
      expect(profileNameSchema.parse('My Server')).toBe('My Server');
      expect(profileNameSchema.parse('Server-123')).toBe('Server-123');
    });

    it('rejects empty names', () => {
      expect(() => profileNameSchema.parse('')).toThrow();
    });

    it('rejects names with invalid characters', () => {
      expect(() => profileNameSchema.parse('server/name')).toThrow();
      expect(() => profileNameSchema.parse('server\\name')).toThrow();
    });

    it('rejects names longer than 64 characters', () => {
      const longName = 'a'.repeat(65);
      expect(() => profileNameSchema.parse(longName)).toThrow();
    });
  });

  describe('difficultySchema', () => {
    it('accepts valid difficulty values', () => {
      expect(difficultySchema.parse(0)).toBe(0);
      expect(difficultySchema.parse(1.0)).toBe(1.0);
      expect(difficultySchema.parse(10)).toBe(10);
      expect(difficultySchema.parse(20.0)).toBe(20.0);
    });

    it('rejects values below 0', () => {
      expect(() => difficultySchema.parse(-0.1)).toThrow();
    });

    it('rejects values above 20', () => {
      expect(() => difficultySchema.parse(20.1)).toThrow();
    });
  });

  describe('maxPlayersSchema', () => {
    it('accepts valid player counts', () => {
      expect(maxPlayersSchema.parse(1)).toBe(1);
      expect(maxPlayersSchema.parse(70)).toBe(70);
      expect(maxPlayersSchema.parse(100)).toBe(100);
    });

    it('rejects non-integer values', () => {
      expect(() => maxPlayersSchema.parse(10.5)).toThrow();
    });

    it('rejects values below 1', () => {
      expect(() => maxPlayersSchema.parse(0)).toThrow();
    });

    it('rejects values above 100', () => {
      expect(() => maxPlayersSchema.parse(101)).toThrow();
    });
  });

  describe('portSchema', () => {
    it('accepts valid ports', () => {
      expect(portSchema.parse(27000)).toBe(27000);
      expect(portSchema.parse(27015)).toBe(27015);
    });

    it('rejects non-integer ports', () => {
      expect(() => portSchema.parse(27000.5)).toThrow();
    });

    it('rejects ports below 27000', () => {
      expect(() => portSchema.parse(26999)).toThrow();
    });

    it('rejects ports above 27015', () => {
      expect(() => portSchema.parse(27016)).toThrow();
    });
  });

  describe('createProfileSchema', () => {
    it('accepts valid profile input', () => {
      const validInput = {
        name: 'Test Server',
        map: 'TheIsland',
        difficulty: 1.0,
        maxPlayers: 70,
        adminPassword: 'secret123',
        port: 27015,
      };
      expect(createProfileSchema.parse(validInput)).toEqual(validInput);
    });

    it('rejects invalid map', () => {
      const invalidInput = {
        name: 'Test Server',
        map: 'InvalidMap',
        difficulty: 1.0,
        maxPlayers: 70,
        adminPassword: 'secret123',
        port: 27015,
      };
      expect(() => createProfileSchema.parse(invalidInput)).toThrow();
    });
  });
});

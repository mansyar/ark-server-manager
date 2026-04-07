import { describe, it, expect } from 'vitest';
import { ARK_MAPS, type ArkMap } from '../profile';

describe('profile types', () => {
  describe('ARK_MAPS', () => {
    it('contains all expected ARK maps', () => {
      const expectedMaps = [
        'TheIsland',
        'TheCenter',
        'ScorchedEarth',
        'Ragnarot',
        'Aberration',
        'Extinction',
        'GenesisPart1',
        'GenesisPart2',
        'Valguero',
        'Hope',
        'LostIsland',
        'Fjordur',
        'Turkey',
      ];
      expect(ARK_MAPS).toEqual(expectedMaps);
    });

    it('has 13 maps', () => {
      expect(ARK_MAPS).toHaveLength(13);
    });

    it('contains ScorchedEarth', () => {
      expect(ARK_MAPS).toContain('ScorchedEarth');
    });
  });

  describe('ArkMap type', () => {
    it('accepts valid map values', () => {
      const validMap: ArkMap = 'Ragnarot';
      expect(ARK_MAPS).toContain(validMap);
    });
  });
});

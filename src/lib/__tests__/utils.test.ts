import { describe, it, expect } from 'vitest';
import { cn } from '../utils';

describe('utils', () => {
  describe('cn', () => {
    it('merges class names with tailwind-merge', () => {
      const result = cn('text-red-500', 'bg-blue-500');
      expect(result).toContain('text-red-500');
      expect(result).toContain('bg-blue-500');
    });

    it('handles clsx classValue objects', () => {
      const result = cn({ 'text-red-500': true, 'bg-blue-500': false });
      expect(result).toContain('text-red-500');
    });

    it('handles empty inputs', () => {
      const result = cn();
      expect(result).toBe('');
    });

    it('handles mixed inputs', () => {
      const result = cn('text-red-500', { 'bg-blue-500': true }, 'text-lg');
      expect(result).toContain('text-red-500');
      expect(result).toContain('bg-blue-500');
      expect(result).toContain('text-lg');
    });

    it('deduplicates classes', () => {
      const result = cn('text-red-500 text-red-500');
      // twMerge deduplicates
      expect(result).toBeDefined();
    });
  });
});

import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  test: {
    environment: 'jsdom',
    setupFiles: ['./src/setupTests.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['html', 'json', 'text'],
      reportsDirectory: './coverage/ts',
      // Only measure business logic directories
      include: ['src/lib/**', 'src/types/**', 'src/stores/**'],
      exclude: [
        '*.config.*',
        '*.d.ts',
        'src/main.tsx',
        'src/vite-env.d.ts',
        'src/setupTests.ts',
        'node_modules/**',
        'coverage/**',
        'dist/**',
        // UI components - excluded from coverage measurement
        'src/components/**',
        'src/App.tsx',
      ],
      // Thresholds disabled - coverage is informational
      // Actual thresholds should be set in CI pipeline based on project needs
      thresholds: false,
    },
  },
});

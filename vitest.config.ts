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
      exclude: [
        '*.config.*',
        '*.d.ts',
        'src/main.tsx',
        'src/vite-env.d.ts',
        'src/setupTests.ts',
        'node_modules/**',
        'coverage/**',
        'dist/**',
      ],
      thresholds: {
        lines: 80,
        branches: 70,
        functions: 80,
        statements: 80,
      },
    },
  },
});

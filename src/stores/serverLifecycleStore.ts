import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
  ServerStatus,
  ServerHandle,
  ConsoleLine,
  PlayerInfo,
  ValidationResult,
  HealthMetrics,
} from '../types/server';

/// Crash data captured when a server crashes.
export interface CrashData {
  profileName: string;
  exitCode: number | null;
  lastLogLines: string[];
  timestamp: Date;
}

interface ServerState {
  // Per-profile server status
  status: Record<string, ServerStatus>;
  // Per-profile server handles
  handles: Record<string, ServerHandle>;
  // Per-profile console buffers
  consoleBuffers: Record<string, ConsoleLine[]>;
  // Per-profile players
  players: Record<string, PlayerInfo[]>;
  // Per-profile last update timestamps
  lastPlayerUpdate: Record<string, Date | null>;
  // Per-profile install validation
  validation: Record<string, ValidationResult>;
  // Loading states
  isStarting: Record<string, boolean>;
  isStopping: Record<string, boolean>;
  // Error states
  errors: Record<string, string | null>;
  // Active profile for detail view
  activeServerProfile: string | null;
  // Unlisten functions
  unlisteners: (() => void)[];
  // Crash data for showing crash dialog
  crashData: Record<string, CrashData>;
  // Whether crash dialog is open
  showCrashDialog: boolean;
  // Crash dialog profile
  crashDialogProfile: string | null;
  // Per-profile health metrics
  healthMetrics: Record<string, HealthMetrics>;
}

interface ServerActions {
  // Initialize event listeners
  initListeners: () => Promise<void>;
  // Cleanup event listeners
  cleanupListeners: () => void;
  // Get server status for a profile
  getStatus: (profileName: string) => ServerStatus;
  // Refresh server status from backend
  refreshStatus: (profileName: string) => Promise<void>;
  // Start server for a profile
  startServer: (profileName: string) => Promise<void>;
  // Stop server for a profile
  stopServer: (profileName: string) => Promise<void>;
  // Restart server for a profile
  restartServer: (profileName: string) => Promise<void>;
  // Validate install for a profile
  validateInstall: (profileName: string) => Promise<ValidationResult>;
  // Get console buffer for a profile
  getConsoleBuffer: (profileName: string) => Promise<ConsoleLine[]>;
  // Clear console buffer for a profile
  clearConsoleBuffer: (profileName: string) => void;
  // Add console line to buffer
  addConsoleLine: (line: ConsoleLine) => void;
  // Get players for a profile
  getPlayers: (profileName: string) => PlayerInfo[];
  // Set active server profile for detail view
  setActiveServerProfile: (profileName: string | null) => void;
  // Show crash dialog for a profile
  showCrashDialogForProfile: (profileName: string) => void;
  // Close crash dialog
  closeCrashDialog: () => void;
}

export const useServerStore = create<ServerState & ServerActions>((set, get) => ({
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
  crashData: {},
  showCrashDialog: false,
  crashDialogProfile: null,
  healthMetrics: {},

  initListeners: async () => {
    const unlisteners: (() => void)[] = [];

    // Listen for server started
    unlisteners.push(
      await listen<ServerHandle>('server-started', (event) => {
        const handle = event.payload;
        set((state) => ({
          status: { ...state.status, [handle.profile_name]: 'Running' },
          handles: { ...state.handles, [handle.profile_name]: handle },
          isStarting: { ...state.isStarting, [handle.profile_name]: false },
          errors: { ...state.errors, [handle.profile_name]: null },
        }));
      })
    );

    // Listen for server stopped
    unlisteners.push(
      await listen<{ profile_name: string }>('server-stopped', (event) => {
        const { profile_name } = event.payload;
        set((state) => ({
          status: { ...state.status, [profile_name]: 'Stopped' },
          isStopping: { ...state.isStopping, [profile_name]: false },
        }));
      })
    );

    // Listen for status changed
    unlisteners.push(
      await listen<{ profile_name: string; status: ServerStatus }>('status-changed', (event) => {
        const { profile_name, status } = event.payload;
        set((state) => ({
          status: { ...state.status, [profile_name]: status },
        }));
      })
    );

    // Listen for server crashed
    unlisteners.push(
      await listen<{ profile_name: string; error: string }>('server-crashed', (event) => {
        const { profile_name, error } = event.payload;
        set((state) => ({
          status: { ...state.status, [profile_name]: 'Crashed' },
          errors: { ...state.errors, [profile_name]: error },
          isStarting: { ...state.isStarting, [profile_name]: false },
          isStopping: { ...state.isStopping, [profile_name]: false },
        }));
      })
    );

    // Listen for crash-detected with detailed crash information
    unlisteners.push(
      await listen<{ profile_name: string; exit_code: number | null; last_log_lines: string[] }>(
        'crash-detected',
        (event) => {
          const { profile_name, exit_code, last_log_lines } = event.payload;
          const crashData: CrashData = {
            profileName: profile_name,
            exitCode: exit_code,
            lastLogLines: last_log_lines,
            timestamp: new Date(),
          };
          set((state) => ({
            status: { ...state.status, [profile_name]: 'Crashed' },
            errors: { ...state.errors, [profile_name]: `Server crashed with exit code ${exit_code ?? 'unknown'}` },
            isStarting: { ...state.isStarting, [profile_name]: false },
            isStopping: { ...state.isStopping, [profile_name]: false },
            crashData: { ...state.crashData, [profile_name]: crashData },
            showCrashDialog: true,
            crashDialogProfile: profile_name,
          }));
        }
      )
    );

    // Listen for console output
    unlisteners.push(
      await listen<ConsoleLine>('console-output', (event) => {
        get().addConsoleLine(event.payload);
      })
    );

    // Listen for player list updated
    unlisteners.push(
      await listen<{ profile_name: string; players: PlayerInfo[] }>(
        'player-list-updated',
        (event) => {
          const { profile_name, players } = event.payload;
          set((state) => ({
            players: { ...state.players, [profile_name]: players },
            lastPlayerUpdate: { ...state.lastPlayerUpdate, [profile_name]: new Date() },
          }));
        }
      )
    );

    // Listen for health update
    unlisteners.push(
      await listen<HealthMetrics>('health-update', (event) => {
        const metrics = event.payload;
        set((state) => ({
          healthMetrics: { ...state.healthMetrics, [metrics.profile_name]: metrics },
        }));
      })
    );

    // Listen for auto-restart-exhausted
    unlisteners.push(
      await listen<string>('auto-restart-exhausted', (event) => {
        const profileName = event.payload;
        set((state) => ({
          errors: {
            ...state.errors,
            [profileName]: `Auto-restart exhausted: Maximum restart attempts reached within 5 minutes.`,
          },
        }));
      })
    );

    set({ unlisteners });
  },

  cleanupListeners: () => {
    const { unlisteners } = get();
    unlisteners.forEach((unlisten) => unlisten());
    set({ unlisteners: [] });
  },

  getStatus: (profileName) => {
    return get().status[profileName] ?? 'Stopped';
  },

  refreshStatus: async (profileName) => {
    try {
      const status = await invoke<ServerStatus>('get_server_status', { profileName });
      set((state) => ({
        status: { ...state.status, [profileName]: status },
      }));
    } catch (e) {
      console.error('Failed to refresh server status:', e);
    }
  },

  startServer: async (profileName) => {
    set((state) => ({
      isStarting: { ...state.isStarting, [profileName]: true },
      errors: { ...state.errors, [profileName]: null },
    }));
    try {
      // Optimistically set to Starting
      set((state) => ({
        status: { ...state.status, [profileName]: 'Starting' },
      }));
      await invoke<ServerHandle>('start_server', { profileName });
      // Status will be updated by server-started event
    } catch (e) {
      set((state) => ({
        status: { ...state.status, [profileName]: 'Stopped' },
        isStarting: { ...state.isStarting, [profileName]: false },
        errors: { ...state.errors, [profileName]: String(e) },
      }));
    }
  },

  stopServer: async (profileName) => {
    set((state) => ({
      isStopping: { ...state.isStopping, [profileName]: true },
    }));
    try {
      await invoke('stop_server', { profileName });
      // Status will be updated by server-stopped event
    } catch (e) {
      set((state) => ({
        isStopping: { ...state.isStopping, [profileName]: false },
        errors: { ...state.errors, [profileName]: String(e) },
      }));
    }
  },

  restartServer: async (profileName) => {
    const { stopServer, startServer } = get();
    await stopServer(profileName);
    // Wait a moment for the server to actually stop
    await new Promise((resolve) => setTimeout(resolve, 2000));
    await startServer(profileName);
  },

  validateInstall: async (profileName) => {
    try {
      const result = await invoke<ValidationResult>('validate_install', { profileName });
      set((state) => ({
        validation: { ...state.validation, [profileName]: result },
      }));
      return result;
    } catch (e) {
      const errorResult: ValidationResult = {
        is_valid: false,
        message: String(e),
        ark_exe_path: null,
      };
      set((state) => ({
        validation: { ...state.validation, [profileName]: errorResult },
      }));
      return errorResult;
    }
  },

  getConsoleBuffer: async (profileName) => {
    try {
      const buffer = await invoke<ConsoleLine[]>('get_console_buffer', { profileName });
      set((state) => ({
        consoleBuffers: { ...state.consoleBuffers, [profileName]: buffer },
      }));
      return buffer;
    } catch (e) {
      console.error('Failed to get console buffer:', e);
      return get().consoleBuffers[profileName] ?? [];
    }
  },

  clearConsoleBuffer: (profileName) => {
    set((state) => ({
      consoleBuffers: { ...state.consoleBuffers, [profileName]: [] },
    }));
  },

  addConsoleLine: (line) => {
    set((state) => {
      const buffer = state.consoleBuffers[line.profile_name] ?? [];
      // Keep last 1000 lines in memory
      const newBuffer = [...buffer, line].slice(-1000);
      return {
        consoleBuffers: { ...state.consoleBuffers, [line.profile_name]: newBuffer },
      };
    });
  },

  getPlayers: (profileName) => {
    return get().players[profileName] ?? [];
  },

  setActiveServerProfile: (profileName) => {
    set({ activeServerProfile: profileName });
  },

  showCrashDialogForProfile: (profileName) => {
    set({ showCrashDialog: true, crashDialogProfile: profileName });
  },

  closeCrashDialog: () => {
    set({ showCrashDialog: false, crashDialogProfile: null });
  },
}));

export const useServerStatus = (profileName: string) =>
  useServerStore((s) => s.status[profileName] ?? 'Stopped');

export const useServerHandle = (profileName: string) =>
  useServerStore((s) => s.handles[profileName] ?? null);

export const useServerPlayers = (profileName: string) =>
  useServerStore((s) => s.players[profileName] ?? []);

export const useConsoleBuffer = (profileName: string) =>
  useServerStore((s) => s.consoleBuffers[profileName] ?? []);

export const useServerValidation = (profileName: string) =>
  useServerStore((s) => s.validation[profileName] ?? null);

export const useHealthMetrics = (profileName: string) =>
  useServerStore((s) => s.healthMetrics[profileName] ?? null);

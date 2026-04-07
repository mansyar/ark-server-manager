export type ServerStatus = 'Stopped' | 'Starting' | 'Running' | 'Stopping' | 'Crashed';

export interface ServerHandle {
  pid: number;
  profile_name: string;
  started_at: string; // ISO string
  ark_exe_path: string;
  port: number;
}

export interface ConsoleLine {
  profile_name: string;
  timestamp: string; // ISO string
  line: string;
  source: 'stdout' | 'stderr';
}

export interface PlayerInfo {
  player_name: string;
  player_id: string;
  tribe: string | null;
  join_time: string; // ISO string
}

export interface ServerInstall {
  steamcmd_path: string | null;
  ark_exe_path: string | null;
  install_dir: string | null;
}

export interface ValidationResult {
  is_valid: boolean;
  message: string | null;
  ark_exe_path: string | null;
}

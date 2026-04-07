export const PROFILE_SCHEMA_VERSION = 1;

export interface Profile {
  schema_version: number;
  name: string;
  map: string;
  difficulty: number;
  max_players: number;
  admin_password: string | null;
  port: number;
  extra_settings: Record<string, string>;
  extra_user_settings: Record<string, string>;
}

export interface ProfileMetadata {
  name: string;
  map: string;
  last_modified: string; // ISO string
}

export const ARK_MAPS = [
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
] as const;

export type ArkMap = (typeof ARK_MAPS)[number];

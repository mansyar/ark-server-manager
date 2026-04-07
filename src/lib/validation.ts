import { z } from 'zod';
import { ARK_MAPS } from '@/types/profile';

// Filename-safe pattern: no / \ : * ? " < > |
const FILENAME_SAFE_PATTERN = /^[^/\\:*?"<>|]+$/;

export const profileNameSchema = z
  .string()
  .min(1, 'Profile name is required')
  .max(64, 'Profile name must be 64 characters or less')
  .regex(FILENAME_SAFE_PATTERN, 'Profile name contains invalid characters');

export const difficultySchema = z
  .number()
  .min(0.0, 'Difficulty must be at least 0')
  .max(20.0, 'Difficulty must be at most 20');

export const maxPlayersSchema = z
  .number()
  .int('Max players must be a whole number')
  .min(1, 'Max players must be at least 1')
  .max(100, 'Max players must be at most 100');

export const adminPasswordSchema = z
  .string()
  .min(4, 'Admin password must be at least 4 characters');

export const portSchema = z
  .number()
  .int('Port must be a whole number')
  .min(27000, 'Port must be at least 27000')
  .max(27015, 'Port must be at most 27015');

export const mapSchema = z.enum(ARK_MAPS as unknown as [string, ...string[]], {
  message: 'Please select a valid ARK map',
});

// Full profile creation schema (wizard steps combined)
export const createProfileSchema = z.object({
  name: profileNameSchema,
  map: mapSchema,
  difficulty: difficultySchema,
  maxPlayers: maxPlayersSchema,
  adminPassword: adminPasswordSchema,
  port: portSchema,
});

// Profile update schema (all fields optional except name)
export const updateProfileSchema = z.object({
  schema_version: z.number().default(1),
  name: z.string().min(1),
  map: z.string(),
  difficulty: z.number(),
  max_players: z.number(),
  admin_password: z.string().nullable(),
  port: z.number(),
  extra_settings: z.record(z.string(), z.string()).default({}),
  extra_user_settings: z.record(z.string(), z.string()).default({}),
});

export type CreateProfileInput = z.infer<typeof createProfileSchema>;
export type UpdateProfileInput = z.infer<typeof updateProfileSchema>;

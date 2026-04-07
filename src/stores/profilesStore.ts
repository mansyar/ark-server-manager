import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { Profile, ProfileMetadata } from '../types/profile';

interface ProfilesState {
  profiles: ProfileMetadata[];
  activeProfile: Profile | null;
  isLoading: boolean;
  error: string | null;
  wizardOpen: boolean;
  editorOpen: boolean;
  loadProfiles: () => Promise<void>;
  createProfile: (profile: Profile) => Promise<void>;
  updateProfile: (profile: Profile) => Promise<void>;
  deleteProfile: (name: string) => Promise<void>;
  setActiveProfile: (profile: Profile | null) => void;
  setWizardOpen: (open: boolean) => void;
  setEditorOpen: (open: boolean) => void;
}

export const useProfilesStore = create<ProfilesState>((set, get) => ({
  profiles: [],
  activeProfile: null,
  isLoading: false,
  error: null,
  wizardOpen: false,
  editorOpen: false,

  loadProfiles: async () => {
    set({ isLoading: true, error: null });
    try {
      const profiles = await invoke<ProfileMetadata[]>('list_profiles');
      set({ profiles, isLoading: false });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  createProfile: async (profile: Profile) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('save_profile', { profile });
      await get().loadProfiles();
      set({ wizardOpen: false });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  updateProfile: async (profile: Profile) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('save_profile', { profile });
      await get().loadProfiles();
      set({ activeProfile: profile });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  deleteProfile: async (name: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('delete_profile', { name });
      await get().loadProfiles();
      set({ activeProfile: null, editorOpen: false });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  setActiveProfile: (profile) => set({ activeProfile: profile }),
  setWizardOpen: (open) => set({ wizardOpen: open }),
  setEditorOpen: (open) => set({ editorOpen: open }),
}));

export const useActiveProfile = () => useProfilesStore((s) => s.activeProfile);

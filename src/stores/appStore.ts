import { create } from 'zustand';

type AppState = object;

export const useAppStore = create<AppState>(() => ({}));

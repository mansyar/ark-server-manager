# Ark Server Manager: UI Design System

This document outlines the core design system and styling principles used to create the modern UI mockup for the Ark Server Manager, strictly aligning with a React + Tailwind CSS + shadcn/ui technology stack.

## 1. Global Themes & Philosophy

- **Aesthetic:** Modern, Premium, Glassmorphic.
- **Design Language:** Windows Fluent Design System inspired (fitting the native Windows desktop target).
- **Core Vibe:** A professional dark-mode-first look that feels inherently "gaming native" due to its dark surface, combined with vibrant, energetic accents.

## 2. Color Palette

The interface relies on Tailwind CSS color tokens.

### Background & Surfaces (zinc/slate scale)
- **Base Background:** `bg-background` (mapped to Tailwind `zinc-950` or `#09090b`). Creates deep contrast.
- **Card / Surface Background:** `bg-card` (mapped to `zinc-900` or `#18181b`). Used for server list cards and the slide-out panel.
- **Muted / Borders:** `bg-muted` (`zinc-800`), `border-border` (`zinc-800`). Used for dividers, disabled states, and subtle container outlines.

### Primary Accents (ARK Amber/Orange scale)
- **Primary Brand:** `bg-primary` (mapped to a custom Amber/Orange, e.g., Tailwind `amber-600` / `#d97706`).
- **Primary Hover/Active:** `hover:bg-amber-500` for interactive action verbs like "Start Server".
- **Primary Glow:** Used in drop shadows for high-priority buttons to give them an energetic "lit-up" look (`shadow-amber-500/20`).

### Semantic Colors
- **Success/Online:** `text-emerald-500` / `bg-emerald-500/10` (for Running status).
- **Error/Offline:** `text-red-500` / `bg-red-500/10` (for Stopped/Crashed status).
- **Warning/Updating:** `text-yellow-500` (for Updating status).

## 3. Typography

- **Font Family:** `font-sans` mapped to "Inter" or "Segoe UI Variable".
- **Hierarchy:**
  - **H1 (App Header):** `text-2xl font-bold tracking-tight`
  - **H2 (Panel Header / Section Titles):** `text-lg font-semibold tracking-tight`
  - **Body (Standard text):** `text-sm text-foreground`
  - **Muted/Helper Text:** `text-xs text-muted-foreground`
  - **Console View:** `font-mono text-xs text-zinc-300` (Consolas or Fira Code).

## 4. Layout Architecture

- **Main Navigation:** Top-aligned Header (`border-b`, `px-4 py-4`, `flex justify-between`).
- **Main Viewport:** A responsive CSS Grid (`grid-cols-1 md:grid-cols-2 lg:grid-cols-3`) containing the Server Profile Cards.
- **Server Details (Slide-out):** An anchored right-side panel mapping to shadcn/ui `Sheet`. Uses `w-full max-w-2xl` to afford space for complex stats and console logs.
- **Spacing:** Base 4px scale. `p-4` or `p-6` for container padding. `gap-4` for element grids.

## 5. UI Components Details

### Cards (shadcn/ui `Card`)
- Used for Server Profiles in the main list and Health Stats in the dashboard.
- Styling: `rounded-xl border bg-card text-card-foreground shadow-sm`.
- Interaction: Main list cards use a subtle `hover:border-primary/50 transition-colors` to indicate they are playable.

### Buttons (shadcn/ui `Button`)
- **Primary (Start):** Filled amber background, bold text, shadow glow. 
- **Secondary (Stop/Edit):** `variant="outline"` or `variant="secondary"`. Clean, less prominent.
- **Ghost (Close/Minimize):** `variant="ghost"`, used for icon buttons in headers to avoid clutter.

### Console Area
- A standard `<ScrollArea>` or `<pre>` tag.
- Styling: `bg-zinc-950 rounded-md border border-zinc-800 p-4 shadow-inner`.
- Provides an authentic "hacker/sysadmin" terminal view that bridges the gap between the GUI and the underlying game server logs.

## 6. Motion & Interaction

- **Fade / Slide In:** The `ServerDetailPanel` relies on Tailwind animate-in tools: `animate-in slide-in-from-right duration-300` providing a smooth, native app feel.
- **Hover Effects:** Almost all interactive elements trigger a visual change (border-color shift or background lighten) to adhere to the principle: *"An interface that feels responsive and alive encourages interaction."*

# TypeScript + React Code Style Guide

## 1. General

- Use **TypeScript strict mode** at all times
- Prefer `interface` over `type` for object shapes
- Use `unknown` instead of `any` for truly unknown types
- No `// @ts-ignore` or `// @ts-nocheck` directives

## 2. Naming Conventions

| Entity | Convention | Example |
|--------|------------|---------|
| Files | kebab-case | `server-profile.ts`, `start-server-button.tsx` |
| Components | PascalCase | `ServerProfileCard.tsx` |
| Functions | camelCase | `createServerProfile()` |
| Hooks | camelCase with `use` prefix | `useServerStatus()` |
| Constants | SCREAMING_SNAKE | `MAX_PLAYERS_DEFAULT` |
| Types/Interfaces | PascalCase | `interface ServerProfile` |
| CSS Classes | kebab-case (Tailwind) | `flex items-center gap-2` |

## 3. React Patterns

### Component Structure
```tsx
// 1. Imports
import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { useServerStore } from '@/stores/server-store';

// 2. Types (if not in separate file)
interface ServerCardProps {
  profile: ServerProfile;
  onStart: (id: string) => void;
}

// 3. Component
export function ServerCard({ profile, onStart }: ServerCardProps) {
  // 3a. Hooks first
  const [isLoading, setIsLoading] = useState(false);

  // 3b. Early returns
  if (!profile) return null;

  // 3c. Main logic
  const handleStart = async () => {
    setIsLoading(true);
    try {
      await onStart(profile.id);
    } finally {
      setIsLoading(false);
    }
  };

  // 3d. Return JSX
  return (
    <div className="rounded-lg border p-4">
      <h3>{profile.name}</h3>
      <Button onClick={handleStart} disabled={isLoading}>
        {isLoading ? 'Starting...' : 'Start'}
      </Button>
    </div>
  );
}
```

### State Management
- Use **Zustand** for global state
- Colocate state — if only one component uses it, keep it local
- Use `useCallback` and `useMemo` sparingly — only when profiling shows it's needed

### File Organization (Frontend)
```
src/
├── components/
│   ├── ui/           # shadcn/ui components (Button, Dialog, etc.)
│   ├── server/       # Server-related components
│   └── settings/     # Settings-related components
├── hooks/            # Custom React hooks
├── stores/           # Zustand stores
├── lib/              # Utilities, Tauri invoke wrappers
├── types/            # TypeScript types/interfaces
└── App.tsx
```

## 4. Tauri IPC

- Wrap all `invoke()` calls in typed functions in `src/lib/commands.ts`
- Never call `invoke()` directly in components — always through command functions
- Handle errors with Result types, never silently swallow

```typescript
// Good
async function startServer(profileId: string): Promise<Result<void, string>> {
  try {
    await invoke('start_server', { profileId });
    return { ok: true, value: undefined };
  } catch (e) {
    return { ok: false, error: String(e) };
  }
}
```

## 5. Styling (Tailwind)

- Use **Tailwind CSS** exclusively for component styling
- Use **shadcn/ui** components as base, customize with Tailwind
- Avoid inline styles except for truly dynamic values
- Prefer `className` over `style` prop

## 6. Error Handling

- Use React Error Boundaries for component tree failures
- Show user-friendly error toasts for recoverable errors
- Log full errors to console AND app log file
- Never expose raw error messages to end users

## 7. Accessibility

- All interactive elements must be keyboard accessible
- Use semantic HTML (`<button>`, `<nav>`, `<main>`)
- ARIA labels on icon-only buttons
- Focus management for modals/dialogs

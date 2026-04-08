import { Eye, EyeOff, Network } from 'lucide-react';
import { cn } from '@/lib/utils';
import { PathInput } from '@/components/ui/PathInput';

interface WizardSecurityStepProps {
  adminPassword: string;
  adminPasswordConfirm: string;
  port: number;
  showPassword: boolean;
  errors: {
    adminPassword?: string[];
    adminPasswordConfirm?: string[];
    port?: string[];
  };
  onAdminPasswordChange: (value: string) => void;
  onAdminPasswordConfirmChange: (value: string) => void;
  onPortChange: (value: number) => void;
  onShowPasswordToggle: () => void;
}

export function WizardSecurityStep({
  adminPassword,
  adminPasswordConfirm,
  port,
  showPassword,
  errors,
  onAdminPasswordChange,
  onAdminPasswordConfirmChange,
  onPortChange,
  onShowPasswordToggle,
}: WizardSecurityStepProps) {
  return (
    <div className="space-y-6">
      {/* Admin Password */}
      <div>
        <label htmlFor="admin-password" className="block text-sm font-medium mb-1.5">
          Admin Password
        </label>
        <div className="relative">
          <input
            id="admin-password"
            type={showPassword ? 'text' : 'password'}
            value={adminPassword}
            onChange={(e) => onAdminPasswordChange(e.target.value)}
            placeholder="Enter admin password"
            className={cn(
              'w-full h-9 rounded-lg border bg-background px-3 pr-10 text-sm transition-colors',
              'focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
              'placeholder:text-muted-foreground',
              errors.adminPassword ? 'border-destructive ring-1 ring-destructive' : 'border-input'
            )}
          />
          <button
            type="button"
            onClick={onShowPasswordToggle}
            className="absolute right-2 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
            aria-label={showPassword ? 'Hide password' : 'Show password'}>
            {showPassword ? <EyeOff className="size-4" /> : <Eye className="size-4" />}
          </button>
        </div>
        {errors.adminPassword && (
          <p className="text-xs text-destructive mt-1">{errors.adminPassword[0]}</p>
        )}
      </div>

      {/* Confirm Admin Password */}
      <div>
        <label htmlFor="admin-password-confirm" className="block text-sm font-medium mb-1.5">
          Confirm Admin Password
        </label>
        <input
          id="admin-password-confirm"
          type={showPassword ? 'text' : 'password'}
          value={adminPasswordConfirm}
          onChange={(e) => onAdminPasswordConfirmChange(e.target.value)}
          placeholder="Confirm admin password"
          className={cn(
            'w-full h-9 rounded-lg border bg-background px-3 text-sm transition-colors',
            'focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
            'placeholder:text-muted-foreground',
            errors.adminPasswordConfirm
              ? 'border-destructive ring-1 ring-destructive'
              : 'border-input'
          )}
        />
        {errors.adminPasswordConfirm && (
          <p className="text-xs text-destructive mt-1">{errors.adminPasswordConfirm[0]}</p>
        )}
      </div>

      {/* Port */}
      <div>
        <label htmlFor="port" className="block text-sm font-medium mb-1.5">
          <Network className="size-3.5 inline mr-1" />
          Server Port
        </label>
        <input
          id="port"
          type="number"
          min={27000}
          max={27015}
          value={port}
          onChange={(e) => onPortChange(parseInt(e.target.value, 10) || 27015)}
          className={cn(
            'w-full h-9 rounded-lg border bg-background px-3 text-sm transition-colors',
            'focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
            errors.port ? 'border-destructive ring-1 ring-destructive' : 'border-input'
          )}
        />
        {errors.port && <p className="text-xs text-destructive mt-1">{errors.port[0]}</p>}
        <p className="text-xs text-muted-foreground mt-1">
          ARK server ports range from 27000 to 27015
        </p>
      </div>
    </div>
  );
}

interface WizardPathsStepProps {
  serverInstallPath: string;
  steamcmdPath: string;
  onServerInstallPathChange: (value: string) => void;
  onSteamcmdPathChange: (value: string) => void;
}

export function WizardPathsStep({
  serverInstallPath,
  steamcmdPath,
  onServerInstallPathChange,
  onSteamcmdPathChange,
}: WizardPathsStepProps) {
  return (
    <div className="space-y-6">
      <p className="text-sm text-muted-foreground mb-4">
        Configure the installation paths for your ARK server. Leave empty to auto-detect.
      </p>

      <PathInput
        label="ARK Server Folder"
        value={serverInstallPath}
        onChange={onServerInstallPathChange}
        pathType="directory"
        placeholder="C:\ARK Server"
        hint="Leave empty to auto-detect"
      />

      <PathInput
        label="SteamCMD Path"
        value={steamcmdPath}
        onChange={onSteamcmdPathChange}
        pathType="file"
        fileFilter="steamcmd.exe"
        placeholder="C:\steamcmd\steamcmd.exe"
        hint="Leave empty to auto-detect"
      />
    </div>
  );
}

interface WizardReviewStepProps {
  name: string;
  map: string;
  difficulty: number;
  maxPlayers: number;
  port: number;
}

export function WizardReviewStep({
  name,
  map,
  difficulty,
  maxPlayers,
  port,
}: WizardReviewStepProps) {
  return (
    <div className="space-y-4">
      <p className="text-sm text-muted-foreground">
        Please review your profile settings before creating:
      </p>

      <div className="rounded-lg border bg-muted/50 p-4 space-y-3">
        <div className="flex justify-between">
          <span className="text-sm text-muted-foreground">Profile Name</span>
          <span className="text-sm font-medium">{name}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-sm text-muted-foreground">Map</span>
          <span className="text-sm font-medium">{map}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-sm text-muted-foreground">Difficulty</span>
          <span className="text-sm font-medium">{difficulty.toFixed(1)}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-sm text-muted-foreground">Max Players</span>
          <span className="text-sm font-medium">{maxPlayers}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-sm text-muted-foreground">Port</span>
          <span className="text-sm font-medium">{port}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-sm text-muted-foreground">Admin Password</span>
          <span className="text-sm font-medium">{'•'.repeat(8)}</span>
        </div>
      </div>
    </div>
  );
}

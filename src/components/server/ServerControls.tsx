import { useEffect } from 'react';
import { Play, Square, RotateCw, AlertTriangle, Download, ExternalLink } from 'lucide-react';
import {
  useServerStore,
  useServerStatus,
  useServerValidation,
} from '@/stores/serverLifecycleStore';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';

interface ServerControlsProps {
  profileName: string;
  variant?: 'card' | 'inline' | 'detail';
  showLabel?: boolean;
}

const statusConfig = {
  Stopped: { color: 'text-red-500', label: 'Stopped', icon: '🔴' },
  Starting: { color: 'text-yellow-500', label: 'Starting', icon: '🟡' },
  Running: { color: 'text-green-500', label: 'Running', icon: '🟢' },
  Stopping: { color: 'text-orange-500', label: 'Stopping', icon: '🟠' },
  Crashed: { color: 'text-destructive', label: 'Crashed', icon: '⚠️' },
};

function ServerControls({ profileName, variant = 'card', showLabel = false }: ServerControlsProps) {
  const status = useServerStatus(profileName);
  const validation = useServerValidation(profileName);
  const { startServer, stopServer, restartServer, validateInstall } = useServerStore();

  useEffect(() => {
    validateInstall(profileName);
  }, [profileName, validateInstall]);

  const handleStart = () => startServer(profileName);
  const handleStop = () => stopServer(profileName);
  const handleRestart = () => restartServer(profileName);

  const statusInfo = statusConfig[status];
  const isRunning = status === 'Running';
  const isStopped = status === 'Stopped';
  const isCrashed = status === 'Crashed';
  const isStartingState = status === 'Starting';
  const isStoppingState = status === 'Stopping';
  const isInstalling = !validation || !validation.is_valid;

  // If binaries not found, show install state
  if (isInstalling && validation && validation.message) {
    return (
      <div className={cn('flex flex-col gap-2', variant === 'card' && 'p-3')}>
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <Download className="size-4" />
          <span>ARK binaries not found</span>
        </div>
        <Button
          variant="outline"
          size="sm"
          onClick={() => {
            // TODO: Link to setup guide
            console.log('Link to setup guide');
          }}
          className="gap-1">
          <ExternalLink className="size-3" />
          Setup Guide
        </Button>
      </div>
    );
  }

  return (
    <div
      className={cn(
        'flex items-center gap-2',
        variant === 'card' && 'p-3',
        variant === 'detail' && 'flex-wrap'
      )}>
      {/* Status indicator */}
      {showLabel && (
        <div className={cn('flex items-center gap-1.5 text-sm', statusInfo.color)}>
          <span>{statusInfo.icon}</span>
          <span className="font-medium">{statusInfo.label}</span>
        </div>
      )}

      {/* Action buttons */}
      <div className="flex items-center gap-1.5">
        {!isRunning && !isStoppingState && (
          <Button
            variant="outline"
            size="icon-sm"
            onClick={handleStart}
            disabled={isStartingState || isStoppingState || isInstalling}
            title={isInstalling ? 'ARK binaries not installed' : 'Start server'}
            className="gap-1">
            {isStartingState ? (
              <RotateCw className="size-3 animate-spin" />
            ) : (
              <Play className="size-3" />
            )}
            {showLabel && <span className="ml-1">{isStartingState ? 'Starting...' : 'Start'}</span>}
          </Button>
        )}

        {isRunning && (
          <Button
            variant="outline"
            size="icon-sm"
            onClick={handleStop}
            disabled={isStoppingState}
            title="Stop server"
            className="gap-1">
            {isStoppingState ? (
              <RotateCw className="size-3 animate-spin" />
            ) : (
              <Square className="size-3" />
            )}
            {showLabel && <span className="ml-1">{isStoppingState ? 'Stopping...' : 'Stop'}</span>}
          </Button>
        )}

        {(isRunning || isCrashed || isStopped) && !isStartingState && !isStoppingState && (
          <Button
            variant="ghost"
            size="icon-sm"
            onClick={handleRestart}
            disabled={isStartingState || isStoppingState || isInstalling}
            title="Restart server"
            className="gap-1">
            <RotateCw className="size-3" />
            {showLabel && <span className="ml-1">Restart</span>}
          </Button>
        )}

        {isCrashed && (
          <div className="flex items-center gap-1 text-xs text-destructive">
            <AlertTriangle className="size-3" />
            <span>Crashed</span>
          </div>
        )}
      </div>
    </div>
  );
}

export { ServerControls, statusConfig };

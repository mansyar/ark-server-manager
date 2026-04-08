import { useEffect } from 'react';
import { Cpu, MemoryStick, Users, Clock, Activity } from 'lucide-react';
import { useHealthMetrics, useServerStatus } from '@/stores/serverLifecycleStore';
import type { ServerStatus } from '@/types/server';

interface HealthDashboardProps {
  profileName: string;
}

/**
 * Formats uptime in seconds to a human-readable string (HH:MM:SS or Xh Xm).
 */
function formatUptime(seconds: number): string {
  if (seconds === 0) return '0s';

  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  if (hours > 0) {
    return `${hours}h ${minutes}m`;
  }
  if (minutes > 0) {
    return `${minutes}m ${secs}s`;
  }
  return `${secs}s`;
}

/**
 * Returns a status badge configuration for display.
 */
function getStatusConfig(status: ServerStatus) {
  switch (status) {
    case 'Running':
      return {
        label: 'Running',
        className: 'bg-green-500/20 text-green-500 border-green-500/30',
      };
    case 'Starting':
      return {
        label: 'Starting',
        className: 'bg-yellow-500/20 text-yellow-500 border-yellow-500/30',
      };
    case 'Stopping':
      return {
        label: 'Stopping',
        className: 'bg-orange-500/20 text-orange-500 border-orange-500/30',
      };
    case 'Crashed':
      return {
        label: 'Crashed',
        className: 'bg-red-500/20 text-red-500 border-red-500/30',
      };
    case 'Stopped':
    default:
      return {
        label: 'Stopped',
        className: 'bg-gray-500/20 text-gray-500 border-gray-500/30',
      };
  }
}

/**
 * Progress bar component for CPU/Memory display.
 */
function MetricBar({
  value,
  max = 100,
  color = 'bg-blue-500',
  label,
  showPercent = true,
}: {
  value: number;
  max?: number;
  color?: string;
  label: string;
  showPercent?: boolean;
}) {
  const percentage = Math.min((value / max) * 100, 100);

  return (
    <div className="space-y-1">
      <div className="flex justify-between text-xs">
        <span className="text-muted-foreground">{label}</span>
        {showPercent && (
          <span className="font-mono">{percentage.toFixed(1)}%</span>
        )}
      </div>
      <div className="h-2 bg-muted rounded-full overflow-hidden">
        <div
          className={`h-full ${color} transition-all duration-300`}
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  );
}

export function HealthDashboard({ profileName }: HealthDashboardProps) {
  const health = useHealthMetrics(profileName);
  const status = useServerStatus(profileName);

  // Use health data for status if available, otherwise fall back to store status
  const currentStatus = health?.status ?? status;
  const statusConfig = getStatusConfig(currentStatus);

  // Auto-format uptime counter every second if running
  useEffect(() => {
    if (currentStatus !== 'Running' || !health) return;

    const interval = setInterval(() => {
      // Force re-render to update uptime display
    }, 1000);

    return () => clearInterval(interval);
  }, [currentStatus, health]);

  return (
    <div className="bg-card border rounded-lg p-4 space-y-4">
      {/* Header with Status Badge */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Activity className="size-4 text-muted-foreground" />
          <span className="text-sm font-medium">Server Health</span>
        </div>
        <span
          className={`text-xs px-2 py-0.5 rounded-full border ${statusConfig.className}`}
        >
          {statusConfig.label}
        </span>
      </div>

      {/* No health data - show placeholder */}
      {!health && currentStatus === 'Stopped' && (
        <div className="text-center py-6 text-muted-foreground text-sm">
          Server is not running
        </div>
      )}

      {/* Health metrics grid */}
      {health && (
        <div className="grid grid-cols-2 gap-4">
          {/* CPU */}
          <div className="space-y-1">
            <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
              <Cpu className="size-3" />
              <span>CPU</span>
            </div>
            <MetricBar
              value={health.cpu_percent}
              max={100}
              color={
                health.cpu_percent > 80
                  ? 'bg-red-500'
                  : health.cpu_percent > 50
                    ? 'bg-yellow-500'
                    : 'bg-green-500'
              }
              label={`${health.cpu_percent.toFixed(1)}%`}
              showPercent={false}
            />
          </div>

          {/* Memory */}
          <div className="space-y-1">
            <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
              <MemoryStick className="size-3" />
              <span>Memory</span>
            </div>
            <MetricBar
              value={health.memory_percent}
              max={100}
              color={
                health.memory_percent > 80
                  ? 'bg-red-500'
                  : health.memory_percent > 50
                    ? 'bg-yellow-500'
                    : 'bg-blue-500'
              }
              label={`${health.memory_mb} MB`}
              showPercent={false}
            />
          </div>

          {/* Players */}
          <div className="space-y-1">
            <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
              <Users className="size-3" />
              <span>Players</span>
            </div>
            <div className="flex items-baseline gap-1">
              <span className="text-lg font-mono font-semibold">
                {health.player_count}
              </span>
              <span className="text-xs text-muted-foreground">
                / {health.max_players}
              </span>
            </div>
          </div>

          {/* Uptime */}
          <div className="space-y-1">
            <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
              <Clock className="size-3" />
              <span>Uptime</span>
            </div>
            <span className="text-lg font-mono font-semibold">
              {formatUptime(health.uptime_seconds)}
            </span>
          </div>
        </div>
      )}
    </div>
  );
}

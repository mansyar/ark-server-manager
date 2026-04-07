import { useEffect, useState } from 'react';
import { Users, Wifi, WifiOff } from 'lucide-react';
import { useServerStore, useServerPlayers } from '@/stores/serverLifecycleStore';
import { cn } from '@/lib/utils';

interface PlayerListProps {
  profileName: string | null;
}

function PlayerList({ profileName }: PlayerListProps) {
  const players = useServerPlayers(profileName ?? '');
  const { getStatus } = useServerStore();
  const [timeSinceUpdate, setTimeSinceUpdate] = useState<string>('');

  const status = profileName ? getStatus(profileName) : 'Stopped';
  const isOffline = status === 'Stopped' || status === 'Crashed';

  // Sort players by join time (newest first)
  const sortedPlayers = [...players].sort(
    (a, b) => new Date(b.join_time).getTime() - new Date(a.join_time).getTime()
  );

  // Update "X seconds ago" indicator
  useEffect(() => {
    if (!profileName) return;

    const interval = setInterval(() => {
      const lastUpdate = useServerStore.getState().lastPlayerUpdate[profileName];
      if (lastUpdate) {
        const seconds = Math.floor((Date.now() - lastUpdate.getTime()) / 1000);
        if (seconds < 60) {
          setTimeSinceUpdate(`${seconds}s ago`);
        } else {
          const minutes = Math.floor(seconds / 60);
          setTimeSinceUpdate(`${minutes}m ago`);
        }
      } else {
        setTimeSinceUpdate('');
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [profileName]);

  const formatJoinTime = (isoString: string) => {
    try {
      return new Date(isoString).toLocaleTimeString();
    } catch {
      return isoString;
    }
  };

  // Offline state
  if (isOffline) {
    return (
      <div className="flex flex-col h-full border rounded-lg bg-card">
        <div className="flex items-center gap-2 px-3 py-2 border-b bg-muted/30">
          <Users className="size-4" />
          <span className="text-sm font-medium">Players</span>
        </div>
        <div className="flex-1 flex items-center justify-center text-muted-foreground">
          <div className="text-center">
            <WifiOff className="size-8 mx-auto mb-2 opacity-50" />
            <p className="text-sm">Server not running</p>
            <p className="text-xs mt-1">RCON unavailable</p>
          </div>
        </div>
      </div>
    );
  }

  // Empty state
  if (players.length === 0) {
    return (
      <div className="flex flex-col h-full border rounded-lg bg-card">
        <div className="flex items-center gap-2 px-3 py-2 border-b bg-muted/30">
          <Users className="size-4" />
          <span className="text-sm font-medium">Players</span>
        </div>
        <div className="flex-1 flex items-center justify-center text-muted-foreground">
          <div className="text-center">
            <Users className="size-8 mx-auto mb-2 opacity-50" />
            <p className="text-sm">No players connected</p>
            <p className="text-xs mt-1">Players will appear here when they join</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full border rounded-lg bg-card">
      {/* Header */}
      <div className="flex items-center justify-between px-3 py-2 border-b bg-muted/30">
        <div className="flex items-center gap-2">
          <Users className="size-4" />
          <span className="text-sm font-medium">Players</span>
          <span className="text-xs text-muted-foreground">({players.length})</span>
        </div>
        <div className="flex items-center gap-1 text-xs text-muted-foreground">
          <Wifi className="size-3" />
          <span>Updated {timeSinceUpdate}</span>
        </div>
      </div>

      {/* Player table */}
      <div className="flex-1 overflow-auto">
        <table className="w-full text-sm">
          <thead className="bg-muted/50 text-muted-foreground sticky top-0">
            <tr>
              <th className="text-left font-medium px-3 py-2">Player Name</th>
              <th className="text-left font-medium px-3 py-2">ID</th>
              <th className="text-left font-medium px-3 py-2">Tribe</th>
              <th className="text-left font-medium px-3 py-2">Join Time</th>
            </tr>
          </thead>
          <tbody>
            {sortedPlayers.map((player, index) => (
              <tr
                key={player.player_id}
                className={cn('border-t', index % 2 === 0 ? 'bg-card' : 'bg-muted/30')}>
                <td className="px-3 py-2 font-medium">{player.player_name}</td>
                <td className="px-3 py-2 text-muted-foreground font-mono text-xs">
                  {player.player_id}
                </td>
                <td className="px-3 py-2">
                  {player.tribe || <span className="text-muted-foreground">—</span>}
                </td>
                <td className="px-3 py-2 text-muted-foreground">
                  {formatJoinTime(player.join_time)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export { PlayerList };

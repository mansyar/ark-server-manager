import { useEffect } from 'react';
import { X, Server as ServerIcon } from 'lucide-react';
import { useServerStore, useServerStatus } from '@/stores/serverLifecycleStore';
import { Button } from '@/components/ui/button';
import { ServerControls } from './ServerControls';
import { ConsoleViewer } from './ConsoleViewer';
import { PlayerList } from './PlayerList';
import { HealthDashboard } from './HealthDashboard';
import { useProfilesStore } from '@/stores/profilesStore';

function ServerDetailPanel() {
  const { activeServerProfile, setActiveServerProfile, refreshStatus } = useServerStore();
  const { setEditorOpen } = useProfilesStore();
  const status = useServerStatus(activeServerProfile ?? '');

  // Refresh status periodically when panel is open
  useEffect(() => {
    if (!activeServerProfile) return;

    refreshStatus(activeServerProfile);
    const interval = setInterval(() => {
      refreshStatus(activeServerProfile);
    }, 5000);

    return () => clearInterval(interval);
  }, [activeServerProfile, refreshStatus]);

  const handleClose = () => {
    setActiveServerProfile(null);
  };

  const handleEditProfile = () => {
    if (activeServerProfile) {
      setEditorOpen(true);
    }
  };

  if (!activeServerProfile) return null;

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black/30 z-40 animate-in fade-in duration-200"
        onClick={handleClose}
      />

      {/* Slide-out panel */}
      <div className="fixed right-0 top-0 bottom-0 w-full max-w-2xl z-50 bg-background border-l shadow-xl animate-in slide-in-from-right duration-300 flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b bg-card">
          <div className="flex items-center gap-3">
            <ServerIcon className="size-5" />
            <div>
              <h2 className="font-semibold">{activeServerProfile}</h2>
              <p className="text-xs text-muted-foreground">
                {status === 'Running'
                  ? `Server running on port ${useServerStore.getState().handles[activeServerProfile]?.port ?? '?'}`
                  : status}
              </p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Button variant="ghost" size="icon-sm" onClick={handleEditProfile}>
              Edit
            </Button>
            <Button variant="ghost" size="icon-sm" onClick={handleClose}>
              <X className="size-4" />
            </Button>
          </div>
        </div>

        {/* Server Controls */}
        <div className="px-4 py-3 border-b bg-muted/30">
          <ServerControls profileName={activeServerProfile} variant="detail" showLabel />
        </div>

        {/* Health Dashboard */}
        <div className="px-4 py-3 border-b">
          <HealthDashboard profileName={activeServerProfile} />
        </div>

        {/* Content: Console + Players in tabs or split view */}
        <div className="flex-1 flex flex-col overflow-hidden">
          <div className="flex-1 overflow-hidden">
            <ConsoleViewer profileName={activeServerProfile} />
          </div>
          <div className="h-64 overflow-hidden border-t">
            <PlayerList profileName={activeServerProfile} />
          </div>
        </div>
      </div>
    </>
  );
}

export { ServerDetailPanel };

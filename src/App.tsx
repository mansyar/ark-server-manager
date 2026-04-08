import { useEffect } from 'react';
import { useProfilesStore } from '@/stores/profilesStore';
import { useServerStore } from '@/stores/serverLifecycleStore';
import { ProfileListView } from '@/components/profiles/ProfileListView';
import { ProfileWizard } from '@/components/profiles/ProfileWizard';
import { ProfileEditor } from '@/components/profiles/ProfileEditor';
import { ServerDetailPanel } from '@/components/server/ServerDetailPanel';
import { CrashDialog } from '@/components/server/CrashDialog';

function App() {
  const { wizardOpen, editorOpen } = useProfilesStore();
  const { initListeners, cleanupListeners, activeServerProfile, showCrashDialog, crashDialogProfile, closeCrashDialog } = useServerStore();

  // Initialize server event listeners on mount
  useEffect(() => {
    initListeners();
    return () => cleanupListeners();
  }, [initListeners, cleanupListeners]);

  return (
    <div className="min-h-screen bg-background text-foreground">
      <header className="border-b border-border">
        <div className="container mx-auto px-4 py-4">
          <h1 className="text-2xl font-bold tracking-tight">Ark Server Manager</h1>
          <p className="text-sm text-muted-foreground">Manage your ARK: Survival Evolved servers</p>
        </div>
      </header>

      <main className="container mx-auto px-4 py-8">
        <ProfileListView />
      </main>

      {wizardOpen && <ProfileWizard />}
      {editorOpen && <ProfileEditor />}
      {activeServerProfile && <ServerDetailPanel />}

      {/* Crash Dialog */}
      {showCrashDialog && crashDialogProfile && (
        <CrashDialog
          profileName={crashDialogProfile}
          onClose={closeCrashDialog}
        />
      )}
    </div>
  );
}

export default App;

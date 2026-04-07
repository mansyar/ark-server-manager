import { useProfilesStore } from '@/stores/profilesStore';
import { ProfileListView } from '@/components/profiles/ProfileListView';
import { ProfileWizard } from '@/components/profiles/ProfileWizard';
import { ProfileEditor } from '@/components/profiles/ProfileEditor';

function App() {
  const { wizardOpen, editorOpen } = useProfilesStore();

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
    </div>
  );
}

export default App;

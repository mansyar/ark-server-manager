import { useEffect, useState } from 'react';
import { Plus, Pencil, Trash2, Map, Clock } from 'lucide-react';
import { useProfilesStore } from '@/stores/profilesStore';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { ConfirmDialog } from '@/components/ui/ConfirmDialog';
import type { ProfileMetadata, Profile } from '@/types/profile';
import { invoke } from '@tauri-apps/api/core';

function ProfileListView() {
  const {
    profiles,
    isLoading,
    error,
    loadProfiles,
    setWizardOpen,
    setActiveProfile,
    setEditorOpen,
    deleteProfile,
  } = useProfilesStore();

  const [deleteConfirmOpen, setDeleteConfirmOpen] = useState(false);
  const [profileToDelete, setProfileToDelete] = useState<string | null>(null);

  useEffect(() => {
    loadProfiles();
  }, [loadProfiles]);

  const handleEditProfile = async (metadata: ProfileMetadata) => {
    try {
      const profile = await invoke<Profile>('load_profile', { name: metadata.name });
      setActiveProfile(profile);
      setEditorOpen(true);
    } catch (e) {
      console.error('Failed to load profile:', e);
    }
  };

  const handleDeleteClick = (name: string) => {
    setProfileToDelete(name);
    setDeleteConfirmOpen(true);
  };

  const handleConfirmDelete = () => {
    if (profileToDelete) {
      deleteProfile(profileToDelete);
    }
  };

  const formatDate = (isoString: string) => {
    try {
      return new Date(isoString).toLocaleDateString(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
      });
    } catch {
      return isoString;
    }
  };

  if (error) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center">
          <p className="text-destructive font-medium">Error loading profiles</p>
          <p className="text-sm text-muted-foreground mt-1">{error}</p>
          <Button variant="outline" className="mt-4" onClick={loadProfiles}>
            Retry
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="container py-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-2xl font-semibold tracking-tight">Server Profiles</h1>
          <p className="text-sm text-muted-foreground mt-1">
            Manage your ARK server configurations
          </p>
        </div>
        <Button onClick={() => setWizardOpen(true)}>
          <Plus className="size-4 mr-1" />
          Create New Profile
        </Button>
      </div>

      {isLoading && profiles.length === 0 ? (
        <div className="flex h-64 items-center justify-center">
          <p className="text-muted-foreground">Loading profiles...</p>
        </div>
      ) : profiles.length === 0 ? (
        <div className="flex h-64 flex-col items-center justify-center rounded-xl border border-dashed text-center">
          <div className="rounded-full bg-muted p-4 mb-4">
            <Map className="size-8 text-muted-foreground" />
          </div>
          <h3 className="font-semibold text-lg">No profiles yet</h3>
          <p className="text-sm text-muted-foreground mt-1 max-w-sm">
            Create your first ARK server profile to get started with configuring your server.
          </p>
          <Button className="mt-6" onClick={() => setWizardOpen(true)}>
            <Plus className="size-4 mr-1" />
            Create Your First Profile
          </Button>
        </div>
      ) : (
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {profiles.map((profile) => (
            <Card key={profile.name} className="group relative">
              <CardHeader>
                <div className="flex items-start justify-between">
                  <div className="min-w-0 flex-1">
                    <CardTitle className="truncate">{profile.name}</CardTitle>
                    <CardDescription className="mt-1 flex items-center gap-1">
                      <Map className="size-3" />
                      {profile.map}
                    </CardDescription>
                  </div>
                  <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                    <Button
                      variant="ghost"
                      size="icon-xs"
                      onClick={() => handleEditProfile(profile)}
                      aria-label="Edit profile">
                      <Pencil className="size-3.5" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon-xs"
                      onClick={() => handleDeleteClick(profile.name)}
                      aria-label="Delete profile"
                      className="text-destructive hover:text-destructive">
                      <Trash2 className="size-3.5" />
                    </Button>
                  </div>
                </div>
              </CardHeader>
              <CardContent>
                <div className="flex items-center gap-1 text-xs text-muted-foreground">
                  <Clock className="size-3" />
                  Last modified: {formatDate(profile.last_modified)}
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}

      <ConfirmDialog
        open={deleteConfirmOpen}
        onOpenChange={setDeleteConfirmOpen}
        title="Delete Profile"
        description={`Are you sure you want to delete "${profileToDelete}"? This action cannot be undone.`}
        confirmText="Delete"
        variant="destructive"
        onConfirm={handleConfirmDelete}
      />
    </div>
  );
}

export { ProfileListView };

import { useState, useEffect } from 'react';
import { X, Save, Eye, Code, Columns, Map, Users, Lock, Network, FolderOpen } from 'lucide-react';
import { useProfilesStore } from '@/stores/profilesStore';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { ARK_MAPS } from '@/types/profile';
import type { Profile, ArkMap } from '@/types/profile';
import { difficultySchema, maxPlayersSchema, portSchema } from '@/lib/validation';
import { PathInput } from '@/components/ui/PathInput';
import { ExtraSettingsEditor } from './ExtraSettingsEditor';

type EditorTab = 'visual' | 'raw' | 'split';

interface EditorFormData {
  name: string;
  map: ArkMap;
  difficulty: number;
  maxPlayers: number;
  adminPassword: string;
  port: number;
  extraSettings: Record<string, string>;
  extraUserSettings: Record<string, string>;
  serverInstallPath: string;
  steamcmdPath: string;
}

function ProfileEditor() {
  const { editorOpen, setEditorOpen, activeProfile, updateProfile } = useProfilesStore();
  const [activeTab, setActiveTab] = useState<EditorTab>('visual');
  const [formData, setFormData] = useState<EditorFormData | null>(null);
  const [rawJson, setRawJson] = useState('');
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [isSaving, setIsSaving] = useState(false);
  const [saveSuccess, setSaveSuccess] = useState(false);

  // Initialize form data when profile changes
  useEffect(() => {
    if (activeProfile && editorOpen) {
      setFormData({
        name: activeProfile.name,
        map: activeProfile.map as ArkMap,
        difficulty: activeProfile.difficulty,
        maxPlayers: activeProfile.max_players,
        adminPassword: activeProfile.admin_password ?? '',
        port: activeProfile.port,
        extraSettings: { ...activeProfile.extra_settings },
        extraUserSettings: { ...activeProfile.extra_user_settings },
        serverInstallPath: activeProfile.server_install_path ?? '',
        steamcmdPath: activeProfile.steamcmd_path ?? '',
      });
      setRawJson(JSON.stringify(activeProfile, null, 2));
      setErrors({});
      setSaveSuccess(false);
    }
  }, [activeProfile, editorOpen]);

  // Close on escape
  useEffect(() => {
    if (!editorOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        setEditorOpen(false);
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [editorOpen, setEditorOpen]);

  if (!editorOpen || !activeProfile || !formData) return null;

  const updateField = (field: keyof EditorFormData, value: unknown) => {
    setFormData((prev) => (prev ? { ...prev, [field]: value } : prev));
    if (errors[field]) {
      setErrors((prev) => ({ ...prev, [field]: '' }));
    }
    setSaveSuccess(false);
  };

  const handleSave = async () => {
    if (!formData) return;

    // Validate fields
    const newErrors: Record<string, string> = {};

    const difficultyResult = difficultySchema.safeParse(formData.difficulty);
    if (!difficultyResult.success) {
      newErrors.difficulty = difficultyResult.error.issues[0]?.message ?? 'Invalid difficulty';
    }

    const maxPlayersResult = maxPlayersSchema.safeParse(formData.maxPlayers);
    if (!maxPlayersResult.success) {
      newErrors.maxPlayers = maxPlayersResult.error.issues[0]?.message ?? 'Invalid max players';
    }

    const portResult = portSchema.safeParse(formData.port);
    if (!portResult.success) {
      newErrors.port = portResult.error.issues[0]?.message ?? 'Invalid port';
    }

    if (formData.adminPassword && formData.adminPassword.length < 4) {
      newErrors.adminPassword = 'Admin password must be at least 4 characters';
    }

    setErrors(newErrors);
    if (Object.keys(newErrors).length > 0) return;

    setIsSaving(true);
    try {
      const profile: Profile = {
        schema_version: activeProfile.schema_version,
        name: formData.name,
        map: formData.map,
        difficulty: formData.difficulty,
        max_players: formData.maxPlayers,
        admin_password: formData.adminPassword || null,
        port: formData.port,
        server_install_path: formData.serverInstallPath || null,
        steamcmd_path: formData.steamcmdPath || null,
        extra_settings: formData.extraSettings,
        extra_user_settings: formData.extraUserSettings,
      };

      await updateProfile(profile);
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch (e) {
      setErrors({ submit: String(e) });
    } finally {
      setIsSaving(false);
    }
  };

  const handleRawJsonChange = (value: string) => {
    setRawJson(value);
    try {
      const parsed = JSON.parse(value) as Profile;
      setFormData({
        name: parsed.name,
        map: parsed.map as ArkMap,
        difficulty: parsed.difficulty,
        maxPlayers: parsed.max_players,
        adminPassword: parsed.admin_password ?? '',
        port: parsed.port,
        extraSettings: parsed.extra_settings ?? {},
        extraUserSettings: parsed.extra_user_settings ?? {},
        serverInstallPath: parsed.server_install_path ?? '',
        steamcmdPath: parsed.steamcmd_path ?? '',
      });
      setErrors({});
    } catch {
      // Invalid JSON, don't update form
    }
    setSaveSuccess(false);
  };

  const tabs: { id: EditorTab; label: string; icon: typeof Eye }[] = [
    { id: 'visual', label: 'Visual', icon: Eye },
    { id: 'raw', label: 'Raw', icon: Code },
    { id: 'split', label: 'Split', icon: Columns },
  ];

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/50 backdrop-blur-sm animate-in fade-in duration-200"
        onClick={() => setEditorOpen(false)}
        aria-hidden="true"
      />

      {/* Dialog */}
      <div
        className={cn(
          'relative z-10 w-full max-w-4xl mx-4 rounded-xl bg-card text-card-foreground shadow-xl',
          'animate-in fade-in zoom-in-95 duration-200',
          'ring-1 ring-foreground/10 flex flex-col max-h-[90vh]'
        )}>
        {/* Header */}
        <div className="flex items-center justify-between p-6 pb-4 border-b">
          <div>
            <h2 className="text-lg font-semibold">Edit Profile: {activeProfile.name}</h2>
            <p className="text-sm text-muted-foreground mt-0.5">
              Modify server configuration settings
            </p>
          </div>
          <div className="flex items-center gap-2">
            {saveSuccess && (
              <span className="text-xs text-primary font-medium animate-in fade-in">
                Saved successfully
              </span>
            )}
            <Button
              variant="ghost"
              size="icon-xs"
              onClick={() => setEditorOpen(false)}
              aria-label="Close">
              <X className="size-4" />
            </Button>
          </div>
        </div>

        {/* Tabs */}
        <div className="flex items-center gap-1 px-6 py-2 border-b">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={cn(
                  'flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium transition-colors',
                  activeTab === tab.id
                    ? 'bg-primary text-primary-foreground'
                    : 'text-muted-foreground hover:bg-muted hover:text-foreground'
                )}>
                <Icon className="size-3.5" />
                {tab.label}
              </button>
            );
          })}
        </div>

        {/* Content */}
        <div className="flex-1 overflow-hidden flex">
          {/* Visual Tab / Split Left */}
          {(activeTab === 'visual' || activeTab === 'split') && (
            <div className={cn('flex-1 overflow-y-auto p-6', activeTab === 'split' && 'border-r')}>
              <div className="space-y-8">
                {/* Basic Settings */}
                <section>
                  <h3 className="text-sm font-semibold flex items-center gap-2 mb-4">
                    <Map className="size-4" />
                    Basic Settings
                  </h3>
                  <div className="space-y-4">
                    <div>
                      <label htmlFor="edit-name" className="block text-sm font-medium mb-1.5">
                        Profile Name
                      </label>
                      <input
                        id="edit-name"
                        type="text"
                        value={formData.name}
                        onChange={(e) => updateField('name', e.target.value)}
                        className="w-full h-9 rounded-lg border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                        maxLength={64}
                      />
                    </div>

                    <div>
                      <label htmlFor="edit-map" className="block text-sm font-medium mb-1.5">
                        ARK Map
                      </label>
                      <select
                        id="edit-map"
                        value={formData.map}
                        onChange={(e) => updateField('map', e.target.value as ArkMap)}
                        className="w-full h-9 rounded-lg border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring">
                        {ARK_MAPS.map((map) => (
                          <option key={map} value={map}>
                            {map}
                          </option>
                        ))}
                      </select>
                    </div>
                  </div>
                </section>

                {/* Server Settings */}
                <section>
                  <h3 className="text-sm font-semibold flex items-center gap-2 mb-4">
                    <Users className="size-4" />
                    Server Settings
                  </h3>
                  <div className="space-y-4">
                    <div>
                      <label htmlFor="edit-difficulty" className="block text-sm font-medium mb-1.5">
                        Difficulty: {formData.difficulty.toFixed(1)}
                      </label>
                      <input
                        id="edit-difficulty"
                        type="range"
                        min="0"
                        max="20"
                        step="0.1"
                        value={formData.difficulty}
                        onChange={(e) => updateField('difficulty', parseFloat(e.target.value))}
                        className="w-full h-2 rounded-full appearance-none bg-muted cursor-pointer accent-primary"
                      />
                      <div className="flex justify-between text-xs text-muted-foreground mt-1">
                        <span>0 (Easy)</span>
                        <span>10 (Normal)</span>
                        <span>20 (Hard)</span>
                      </div>
                      {errors.difficulty && (
                        <p className="text-xs text-destructive mt-1">{errors.difficulty}</p>
                      )}
                    </div>

                    <div>
                      <label
                        htmlFor="edit-max-players"
                        className="block text-sm font-medium mb-1.5">
                        Max Players: {formData.maxPlayers}
                      </label>
                      <input
                        id="edit-max-players"
                        type="range"
                        min="1"
                        max="100"
                        value={formData.maxPlayers}
                        onChange={(e) => updateField('maxPlayers', parseInt(e.target.value, 10))}
                        className="w-full h-2 rounded-full appearance-none bg-muted cursor-pointer accent-primary"
                      />
                      <div className="flex justify-between text-xs text-muted-foreground mt-1">
                        <span>1</span>
                        <span>100</span>
                      </div>
                      {errors.maxPlayers && (
                        <p className="text-xs text-destructive mt-1">{errors.maxPlayers}</p>
                      )}
                    </div>
                  </div>
                </section>

                {/* Security Settings */}
                <section>
                  <h3 className="text-sm font-semibold flex items-center gap-2 mb-4">
                    <Lock className="size-4" />
                    Security
                  </h3>
                  <div className="space-y-4">
                    <div>
                      <label
                        htmlFor="edit-admin-password"
                        className="block text-sm font-medium mb-1.5">
                        Admin Password
                      </label>
                      <input
                        id="edit-admin-password"
                        type="password"
                        value={formData.adminPassword}
                        onChange={(e) => updateField('adminPassword', e.target.value)}
                        placeholder="Leave blank to keep current"
                        className="w-full h-9 rounded-lg border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring placeholder:text-muted-foreground"
                      />
                      {errors.adminPassword && (
                        <p className="text-xs text-destructive mt-1">{errors.adminPassword}</p>
                      )}
                    </div>
                  </div>
                </section>

                {/* Network Settings */}
                <section>
                  <h3 className="text-sm font-semibold flex items-center gap-2 mb-4">
                    <Network className="size-4" />
                    Network
                  </h3>
                  <div className="space-y-4">
                    <div>
                      <label htmlFor="edit-port" className="block text-sm font-medium mb-1.5">
                        Server Port
                      </label>
                      <input
                        id="edit-port"
                        type="number"
                        min={27000}
                        max={27015}
                        value={formData.port}
                        onChange={(e) => updateField('port', parseInt(e.target.value, 10) || 27015)}
                        className="w-full h-9 rounded-lg border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                      />
                      {errors.port && (
                        <p className="text-xs text-destructive mt-1">{errors.port}</p>
                      )}
                      <p className="text-xs text-muted-foreground mt-1">
                        ARK server ports range from 27000 to 27015
                      </p>
                    </div>
                  </div>
                </section>

                {/* Paths */}
                <section>
                  <h3 className="text-sm font-semibold flex items-center gap-2 mb-4">
                    <FolderOpen className="size-4" />
                    Paths
                  </h3>
                  <div className="space-y-4">
                    <PathInput
                      label="ARK Server Folder"
                      value={formData.serverInstallPath}
                      onChange={(value) => updateField('serverInstallPath', value)}
                      pathType="directory"
                      placeholder="C:\ARK Server"
                      hint="Leave empty to auto-detect"
                    />
                    <PathInput
                      label="SteamCMD Path"
                      value={formData.steamcmdPath}
                      onChange={(value) => updateField('steamcmdPath', value)}
                      pathType="file"
                      fileFilter="steamcmd.exe"
                      placeholder="C:\steamcmd\steamcmd.exe"
                      hint="Leave empty to auto-detect"
                    />
                  </div>
                </section>

                {/* Extra Settings */}
                <ExtraSettingsEditor
                  extraSettings={formData.extraSettings}
                  extraUserSettings={formData.extraUserSettings}
                  onExtraSettingsChange={(s) => updateField('extraSettings', s)}
                  onExtraUserSettingsChange={(s) => updateField('extraUserSettings', s)}
                />
              </div>
            </div>
          )}

          {/* Raw Tab / Split Right */}
          {(activeTab === 'raw' || activeTab === 'split') && (
            <div
              className={cn(
                'flex-1 overflow-hidden flex flex-col',
                activeTab === 'split' && 'p-6'
              )}>
              {activeTab === 'split' ? (
                <div className="space-y-2">
                  <h3 className="text-sm font-semibold">Raw JSON</h3>
                  <textarea
                    value={rawJson}
                    onChange={(e) => handleRawJsonChange(e.target.value)}
                    className="w-full h-[calc(100%-2rem)] rounded-lg border border-input bg-muted/50 p-3 text-xs font-mono focus:outline-none focus:ring-2 focus:ring-ring resize-none"
                  />
                </div>
              ) : (
                <div className="flex-1 overflow-y-auto p-6">
                  <div className="space-y-4">
                    <div>
                      <label htmlFor="raw-json" className="block text-sm font-medium mb-1.5">
                        Profile JSON
                      </label>
                      <textarea
                        id="raw-json"
                        value={rawJson}
                        onChange={(e) => handleRawJsonChange(e.target.value)}
                        className="w-full h-96 rounded-lg border border-input bg-muted/50 p-3 text-xs font-mono focus:outline-none focus:ring-2 focus:ring-ring resize-none"
                      />
                      <p className="text-xs text-muted-foreground mt-1">
                        Edit the raw profile JSON. Changes will be reflected in the Visual tab.
                      </p>
                    </div>
                  </div>
                </div>
              )}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between p-6 pt-4 border-t">
          <div>{errors.submit && <p className="text-sm text-destructive">{errors.submit}</p>}</div>
          <div className="flex items-center gap-2">
            <Button variant="outline" onClick={() => setEditorOpen(false)}>
              Cancel
            </Button>
            <Button onClick={handleSave} disabled={isSaving}>
              <Save className="size-4 mr-1" />
              {isSaving ? 'Saving...' : 'Save Changes'}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

export { ProfileEditor };

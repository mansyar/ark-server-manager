import { X } from 'lucide-react';
import { Button } from '@/components/ui/button';

interface ExtraSettingsEditorProps {
  extraSettings: Record<string, string>;
  extraUserSettings: Record<string, string>;
  onExtraSettingsChange: (settings: Record<string, string>) => void;
  onExtraUserSettingsChange: (settings: Record<string, string>) => void;
}

export function ExtraSettingsEditor({
  extraSettings,
  extraUserSettings,
  onExtraSettingsChange,
  onExtraUserSettingsChange,
}: ExtraSettingsEditorProps) {
  return (
    <>
      {/* Extra Settings */}
      <section>
        <h3 className="text-sm font-semibold mb-4">Extra Settings</h3>
        <div className="space-y-3">
          <p className="text-xs text-muted-foreground">Additional Game.ini settings (advanced)</p>
          {Object.entries(extraSettings).map(([key, value]) => (
            <div key={key} className="flex gap-2">
              <input
                type="text"
                value={key}
                onChange={(e) => {
                  const newSettings = { ...extraSettings };
                  const oldValue = newSettings[key];
                  delete newSettings[key];
                  newSettings[e.target.value] = oldValue;
                  onExtraSettingsChange(newSettings);
                }}
                placeholder="Setting"
                className="flex-1 h-9 rounded-lg border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
              />
              <input
                type="text"
                value={value}
                onChange={(e) => {
                  onExtraSettingsChange({
                    ...extraSettings,
                    [key]: e.target.value,
                  });
                }}
                placeholder="Value"
                className="flex-1 h-9 rounded-lg border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
              />
              <Button
                variant="ghost"
                size="icon-sm"
                onClick={() => {
                  const newSettings = { ...extraSettings };
                  delete newSettings[key];
                  onExtraSettingsChange(newSettings);
                }}>
                <X className="size-3.5" />
              </Button>
            </div>
          ))}
          <Button
            variant="outline"
            size="sm"
            onClick={() => {
              const newKey = `Setting_${Object.keys(extraSettings).length + 1}`;
              onExtraSettingsChange({ ...extraSettings, [newKey]: '' });
            }}>
            Add Setting
          </Button>
        </div>
      </section>

      {/* Extra User Settings */}
      <section>
        <h3 className="text-sm font-semibold mb-4">Extra User Settings</h3>
        <div className="space-y-3">
          <p className="text-xs text-muted-foreground">
            Additional GameUserSettings.ini settings (advanced)
          </p>
          {Object.entries(extraUserSettings).map(([key, value]) => (
            <div key={key} className="flex gap-2">
              <input
                type="text"
                value={key}
                onChange={(e) => {
                  const newSettings = { ...extraUserSettings };
                  const oldValue = newSettings[key];
                  delete newSettings[key];
                  newSettings[e.target.value] = oldValue;
                  onExtraUserSettingsChange(newSettings);
                }}
                placeholder="Setting"
                className="flex-1 h-9 rounded-lg border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
              />
              <input
                type="text"
                value={value}
                onChange={(e) => {
                  onExtraUserSettingsChange({
                    ...extraUserSettings,
                    [key]: e.target.value,
                  });
                }}
                placeholder="Value"
                className="flex-1 h-9 rounded-lg border border-input bg-background px-3 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
              />
              <Button
                variant="ghost"
                size="icon-sm"
                onClick={() => {
                  const newSettings = { ...extraUserSettings };
                  delete newSettings[key];
                  onExtraUserSettingsChange(newSettings);
                }}>
                <X className="size-3.5" />
              </Button>
            </div>
          ))}
          <Button
            variant="outline"
            size="sm"
            onClick={() => {
              const newKey = `UserSetting_${Object.keys(extraUserSettings).length + 1}`;
              onExtraUserSettingsChange({
                ...extraUserSettings,
                [newKey]: '',
              });
            }}>
            Add User Setting
          </Button>
        </div>
      </section>
    </>
  );
}

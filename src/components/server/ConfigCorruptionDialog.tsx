import { AlertTriangle, RotateCcw } from 'lucide-react';
import { ErrorDialog, ErrorDialogProps } from './ErrorDialog';
import { Button } from '@/components/ui/button';

interface ConfigCorruptionDialogProps extends Omit<ErrorDialogProps, 'title' | 'description' | 'icon' | 'variant'> {
  configFile?: string;
  onRestoreBackup?: () => void;
  backupAvailable?: boolean;
}

function ConfigCorruptionDialog({
  configFile,
  onRestoreBackup,
  backupAvailable = false,
  ...props
}: ConfigCorruptionDialogProps) {
  const handleRestoreBackup = () => {
    props.onOpenChange(false);
    onRestoreBackup?.();
  };

  return (
    <ErrorDialog
      {...props}
      title="Configuration File Corrupted"
      description={
        configFile
          ? `The configuration file "${configFile}" appears to be corrupted.`
          : 'A server configuration file appears to be corrupted.'
      }
      icon={<AlertTriangle className="size-5 text-destructive" />}
      variant="destructive"
      actions={
        <div className="flex items-center gap-2">
          <Button variant="outline" onClick={() => props.onOpenChange(false)}>
            Cancel
          </Button>
          {backupAvailable && onRestoreBackup && (
            <Button variant="default" onClick={handleRestoreBackup} className="gap-1.5">
              <RotateCcw className="size-3.5" />
              Restore from Backup
            </Button>
          )}
        </div>
      }
      details={`The server configuration file is invalid or corrupted and cannot be parsed.

This can happen due to:
- Unexpected shutdown during save
- Disk write errors
- Manual editing mistakes
- Software bugs

${
  backupAvailable
    ? 'A backup of this configuration file is available. You can restore it to recover your settings.'
    : 'No backup is available. You may need to recreate the configuration manually or reinstall the server.'
}

If the problem persists, try:
1. Verifying ARK server files through Steam
2. Reinstalling the server profile
3. Checking disk health`}
    />
  );
}

export { ConfigCorruptionDialog };

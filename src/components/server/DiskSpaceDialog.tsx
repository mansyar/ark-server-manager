import { HardDrive } from 'lucide-react';
import { ErrorDialog, ErrorDialogProps } from './ErrorDialog';
import { Button } from '@/components/ui/button';

interface DiskSpaceDialogProps extends Omit<ErrorDialogProps, 'title' | 'description' | 'icon' | 'variant'> {
  requiredBytes?: number;
  availableBytes?: number;
  onCleanup?: () => void;
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

function DiskSpaceDialog({
  requiredBytes,
  availableBytes,
  onCleanup,
  ...props
}: DiskSpaceDialogProps) {
  const handleCleanup = () => {
    props.onOpenChange(false);
    onCleanup?.();
  };

  return (
    <ErrorDialog
      {...props}
      title="Insufficient Disk Space"
      description={
        requiredBytes && availableBytes
          ? `Need ${formatBytes(requiredBytes)} but only ${formatBytes(availableBytes)} available.`
          : 'Not enough disk space to complete this operation.'
      }
      icon={<HardDrive className="size-5 text-yellow-500" />}
      variant="warning"
      actions={
        <div className="flex items-center gap-2">
          <Button variant="outline" onClick={() => props.onOpenChange(false)}>
            Cancel
          </Button>
          {onCleanup && (
            <Button variant="default" onClick={handleCleanup}>
              Cleanup Disk
            </Button>
          )}
        </div>
      }
      details={`To free up disk space, try:
1. Deleting old backup files in %APPDATA%\\ArkServerManager\\backups\\
2. Removing unused ARK server profiles
3. Clearing the Steam download cache
4. Uninstalling unused games or applications

Recommended minimum free space: 10 GB for ARK server operations.`}
    />
  );
}

export { DiskSpaceDialog };

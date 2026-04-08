import { useState } from 'react';
import { Network, RefreshCw } from 'lucide-react';
import { ErrorDialog, ErrorDialogProps } from './ErrorDialog';
import { Button } from '@/components/ui/button';

interface PortConflictDialogProps extends Omit<ErrorDialogProps, 'title' | 'description' | 'icon' | 'variant'> {
  port: number;
  onChangePort?: () => void;
}

function PortConflictDialog({ port, onChangePort, ...props }: PortConflictDialogProps) {
  const [retrying, setRetrying] = useState(false);

  const handleRetry = async () => {
    setRetrying(true);
    // Wait a bit for the port to be released
    await new Promise((resolve) => setTimeout(resolve, 2000));
    setRetrying(false);
    props.onOpenChange(false);
  };

  return (
    <ErrorDialog
      {...props}
      title="Port Conflict Detected"
      description={`Port ${port} is already in use by another application.`}
      icon={<Network className="size-5 text-destructive" />}
      variant="destructive"
      actions={
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            onClick={handleRetry}
            disabled={retrying}
            className="gap-1.5">
            <RefreshCw className={`size-3.5 ${retrying ? 'animate-spin' : ''}`} />
            {retrying ? 'Retrying...' : 'Retry'}
          </Button>
          {onChangePort && (
            <Button variant="outline" onClick={onChangePort}>
              Change Port
            </Button>
          )}
        </div>
      }
      details={`The ARK server cannot bind to port ${port} because another process is using it.

To resolve this issue:
1. Close the application using port ${port}
2. Or change the server port in profile settings to a different value (e.g., ${port + 1})

Common applications that use ports:
- Other game servers
- Web servers (IIS, Apache, Nginx)
- Skype, Discord, or other VoIP applications
- VPN software`}
    />
  );
}

export { PortConflictDialog };

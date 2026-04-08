import { useState } from 'react';
import { Download, RefreshCw } from 'lucide-react';
import { ErrorDialog, ErrorDialogProps } from './ErrorDialog';
import { Button } from '@/components/ui/button';

interface SteamCMDDownloadDialogProps
  extends Omit<ErrorDialogProps, 'title' | 'description' | 'icon' | 'variant'> {
  onRetry?: () => void;
  error?: string;
}

function SteamCMDDownloadDialog({ onRetry, error, ...props }: SteamCMDDownloadDialogProps) {
  const [retrying, setRetrying] = useState(false);

  const handleRetry = async () => {
    setRetrying(true);
    try {
      onRetry?.();
    } finally {
      setRetrying(false);
      props.onOpenChange(false);
    }
  };

  return (
    <ErrorDialog
      {...props}
      title="SteamCMD Download Failed"
      description="Failed to download or update ARK server files. Please check your internet connection and try again."
      icon={<Download className="size-5 text-destructive" />}
      variant="destructive"
      actions={
        <div className="flex items-center gap-2">
          <Button
            variant="default"
            onClick={handleRetry}
            disabled={retrying}
            className="gap-1.5">
            <RefreshCw className={`size-3.5 ${retrying ? 'animate-spin' : ''}`} />
            {retrying ? 'Retrying...' : 'Retry Download'}
          </Button>
          <Button variant="outline" onClick={() => props.onOpenChange(false)}>
            Cancel
          </Button>
        </div>
      }
      details={`SteamCMD failed to download the ARK server files.

Error: ${error || 'Unknown error'}

To manually install SteamCMD:
1. Download SteamCMD from: https://developer.valvesoftware.com/wiki/SteamCMD#Windows
2. Extract to: C:\\steamcmd\\steamcmd.exe
3. Run: steamcmd +login anonymous +force_install_dir C:\\ark_server\\ +app_update 376030 +quit

Common causes:
- Firewall or antivirus blocking Steam
- Slow or unstable internet connection
- Steam servers being overloaded
- Insufficient disk space`}
    />
  );
}

export { SteamCMDDownloadDialog };

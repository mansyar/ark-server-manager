import { useEffect, useRef } from 'react';
import { AlertTriangle, FileText, FolderOpen, X } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { useServerStore } from '@/stores/serverLifecycleStore';
import { cn } from '@/lib/utils';

interface CrashDialogProps {
  profileName: string;
  onClose: () => void;
}

function CrashDialog({ profileName, onClose }: CrashDialogProps) {
  const crashData = useServerStore((s) => s.crashData[profileName]);
  const dialogRef = useRef<HTMLDivElement>(null);

  // Close on escape key
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [onClose]);

  // Focus trap
  useEffect(() => {
    if (!dialogRef.current) return;
    const dialog = dialogRef.current;
    const focusableElements = dialog.querySelectorAll<HTMLElement>(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    const firstElement = focusableElements[0];
    const lastElement = focusableElements[focusableElements.length - 1];
    firstElement?.focus();

    const handleTabFocus = (e: KeyboardEvent) => {
      if (e.key !== 'Tab') return;
      if (e.shiftKey) {
        if (document.activeElement === firstElement) {
          e.preventDefault();
          lastElement?.focus();
        }
      } else {
        if (document.activeElement === lastElement) {
          e.preventDefault();
          firstElement?.focus();
        }
      }
    };
    dialog.addEventListener('keydown', handleTabFocus);
    return () => dialog.removeEventListener('keydown', handleTabFocus);
  }, []);

  const handleOpenLogsFolder = () => {
    // Open logs folder using tauri shell
    // For now, just log
    console.log('Open logs folder for crash reports');
  };

  const handleViewCrashReport = () => {
    // Open the crash report JSON file
    console.log('View crash report for', profileName);
  };

  const logLines = crashData?.lastLogLines ?? [];

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/50 backdrop-blur-sm animate-in fade-in duration-200"
        onClick={onClose}
        aria-hidden="true"
      />

      {/* Dialog */}
      <div
        ref={dialogRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="crash-dialog-title"
        className={cn(
          'relative z-10 w-full max-w-2xl mx-4 rounded-xl bg-card text-card-foreground shadow-xl',
          'animate-in fade-in zoom-in-95 duration-200',
          'ring-1 ring-foreground/10 max-h-[80vh] flex flex-col'
        )}>
        {/* Header */}
        <div className="flex items-center gap-3 p-6 border-b">
          <div className="p-2 rounded-full bg-destructive/10">
            <AlertTriangle className="size-6 text-destructive" />
          </div>
          <div className="flex-1">
            <h2 id="crash-dialog-title" className="text-lg font-semibold leading-snug">
              Server Crashed
            </h2>
            <p className="text-sm text-muted-foreground">
              {profileName} crashed with exit code {crashData?.exitCode ?? 'unknown'}
            </p>
          </div>
          <Button variant="ghost" size="icon-sm" onClick={onClose}>
            <X className="size-4" />
          </Button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-auto p-6">
          <div className="space-y-4">
            {/* Crash details */}
            <div className="text-sm">
              <p className="font-medium">Crash Time</p>
              <p className="text-muted-foreground">
                {crashData?.timestamp
                  ? new Date(crashData.timestamp).toLocaleString()
                  : 'Unknown'}
              </p>
            </div>

            {/* Last log lines */}
            <div>
              <p className="text-sm font-medium mb-2">Last 50 Log Lines</p>
              <div className="bg-muted rounded-lg p-3 font-mono text-xs overflow-auto max-h-64">
                {logLines.length > 0 ? (
                  logLines.map((line, i) => (
                    <div key={i} className="whitespace-pre-wrap break-all">
                      {line}
                    </div>
                  ))
                ) : (
                  <p className="text-muted-foreground">No log lines available</p>
                )}
              </div>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between gap-2 px-6 py-4 border-t bg-muted/30">
          <div className="flex items-center gap-2">
            <Button variant="outline" size="sm" onClick={handleOpenLogsFolder} className="gap-1.5">
              <FolderOpen className="size-3.5" />
              Open Logs Folder
            </Button>
            <Button variant="outline" size="sm" onClick={handleViewCrashReport} className="gap-1.5">
              <FileText className="size-3.5" />
              View Crash Report
            </Button>
          </div>
          <Button variant="default" onClick={onClose}>
            Close
          </Button>
        </div>
      </div>
    </div>
  );
}

export { CrashDialog };

import { useEffect, useRef, useState, useCallback } from 'react';
import { useServerStore, useConsoleBuffer } from '@/stores/serverLifecycleStore';
import { Button } from '@/components/ui/button';
import { Play, Pause, Trash2 } from 'lucide-react';
import { cn } from '@/lib/utils';

interface ConsoleViewerProps {
  profileName: string | null;
  maxLines?: number;
}

function ConsoleViewer({ profileName, maxLines = 100 }: ConsoleViewerProps) {
  const [isFollowing, setIsFollowing] = useState(true);
  const consoleBuffer = useConsoleBuffer(profileName ?? '');
  const { getConsoleBuffer, clearConsoleBuffer, getStatus } = useServerStore();
  const preRef = useRef<HTMLPreElement>(null);

  // Initial load of console buffer
  useEffect(() => {
    if (profileName) {
      getConsoleBuffer(profileName);
    }
  }, [profileName, getConsoleBuffer]);

  // Auto-scroll to bottom when new lines arrive
  useEffect(() => {
    if (!preRef.current) return;

    const pre = preRef.current;
    const isAtBottom = pre.scrollHeight - pre.scrollTop - pre.clientHeight < 50;

    if (isFollowing && isAtBottom) {
      pre.scrollTop = pre.scrollHeight;
    }
  }, [consoleBuffer, isFollowing]);

  // Handle manual scroll
  const handleScroll = useCallback(() => {
    if (!preRef.current) return;

    const pre = preRef.current;
    const isAtBottom = pre.scrollHeight - pre.scrollTop - pre.clientHeight < 50;

    if (!isAtBottom && isFollowing) {
      setIsFollowing(false);
    }

    // Re-enable following if scrolled back to bottom
    if (isAtBottom && !isFollowing) {
      setIsFollowing(true);
    }
  }, [isFollowing]);

  const toggleFollow = () => {
    setIsFollowing(!isFollowing);
    if (!isFollowing && preRef.current) {
      preRef.current.scrollTop = preRef.current.scrollHeight;
    }
  };

  const handleClear = () => {
    if (profileName) {
      clearConsoleBuffer(profileName);
    }
  };

  const status = profileName ? getStatus(profileName) : 'Stopped';
  const isServerStopped = status === 'Stopped';

  // Filter lines based on maxLines and status
  const displayLines =
    isServerStopped && consoleBuffer.length > 0
      ? consoleBuffer.slice(-50) // Show last 50 lines when stopped
      : consoleBuffer.slice(-maxLines);

  return (
    <div className="flex flex-col h-full border rounded-lg bg-card">
      {/* Toolbar */}
      <div className="flex items-center justify-between px-3 py-2 border-b bg-muted/30">
        <div className="flex items-center gap-2">
          <span className="text-sm font-medium">Console</span>
          {isServerStopped && consoleBuffer.length > 0 && (
            <span className="text-xs text-muted-foreground">Server stopped</span>
          )}
        </div>

        <div className="flex items-center gap-1">
          <Button
            variant="ghost"
            size="icon-xs"
            onClick={handleClear}
            title="Clear console"
            disabled={consoleBuffer.length === 0}>
            <Trash2 className="size-3" />
          </Button>
          <Button
            variant={isFollowing ? 'default' : 'outline'}
            size="icon-xs"
            onClick={toggleFollow}
            title={isFollowing ? 'Pause following' : 'Resume following'}
            className="gap-1">
            {isFollowing ? <Pause className="size-3" /> : <Play className="size-3" />}
          </Button>
        </div>
      </div>

      {/* Console output */}
      <div className="flex-1 overflow-auto p-3 font-mono text-xs">
        {consoleBuffer.length === 0 ? (
          <div className="flex h-full items-center justify-center text-muted-foreground">
            <div className="text-center">
              <p className="text-sm">No console output</p>
              <p className="text-xs mt-1">
                {profileName
                  ? 'Start the server to see console output'
                  : 'Select a profile to view console'}
              </p>
            </div>
          </div>
        ) : (
          <pre ref={preRef} className="whitespace-pre-wrap break-all" onScroll={handleScroll}>
            {displayLines.map((line, index) => (
              <div
                key={index}
                className={cn('leading-relaxed', line.source === 'stderr' && 'text-destructive')}>
                <span className="text-muted-foreground mr-2">
                  [{new Date(line.timestamp).toLocaleTimeString()}]
                </span>
                {line.line}
              </div>
            ))}
            {isServerStopped && (
              <div className="mt-2 text-muted-foreground border-t pt-2">--- Server stopped ---</div>
            )}
          </pre>
        )}
      </div>
    </div>
  );
}

export { ConsoleViewer };

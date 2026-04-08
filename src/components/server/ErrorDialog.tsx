import { useEffect, useRef, ReactNode } from 'react';
import { AlertTriangle, X } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';

interface ErrorDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  description?: string;
  details?: string;
  icon?: ReactNode;
  variant?: 'default' | 'destructive' | 'warning';
  actions?: ReactNode;
}

function ErrorDialog({
  open,
  onOpenChange,
  title,
  description,
  details,
  icon,
  variant = 'destructive',
  actions,
}: ErrorDialogProps) {
  const dialogRef = useRef<HTMLDivElement>(null);

  // Close on escape key
  useEffect(() => {
    if (!open) return;
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onOpenChange(false);
      }
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [open, onOpenChange]);

  // Focus trap
  useEffect(() => {
    if (!open || !dialogRef.current) return;
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
  }, [open]);

  if (!open) return null;

  const DefaultIcon = variant === 'destructive' ? AlertTriangle : null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/50 backdrop-blur-sm animate-in fade-in duration-200"
        onClick={() => onOpenChange(false)}
        aria-hidden="true"
      />

      {/* Dialog */}
      <div
        ref={dialogRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="error-dialog-title"
        className={cn(
          'relative z-10 w-full max-w-md mx-4 rounded-xl bg-card text-card-foreground shadow-xl',
          'animate-in fade-in zoom-in-95 duration-200',
          'ring-1 ring-foreground/10'
        )}>
        <div className="p-6">
          <div className="flex items-start gap-3">
            {(icon || DefaultIcon) && (
              <div
                className={cn(
                  'p-2 rounded-full',
                  variant === 'destructive' && 'bg-destructive/10',
                  variant === 'warning' && 'bg-yellow-500/10',
                  variant === 'default' && 'bg-muted'
                )}>
                {icon || (DefaultIcon && (
                  <DefaultIcon className={cn('size-5', variant === 'warning' && 'text-yellow-500')} />
                ))}
              </div>
            )}
            <div className="flex-1">
              <h2 id="error-dialog-title" className="text-lg font-semibold leading-snug">
                {title}
              </h2>
              {description && <p className="mt-2 text-sm text-muted-foreground">{description}</p>}
              {details && (
                <pre className="mt-3 p-3 bg-muted rounded-lg text-xs overflow-auto max-h-32 font-mono">
                  {details}
                </pre>
              )}
            </div>
            <Button variant="ghost" size="icon-sm" onClick={() => onOpenChange(false)}>
              <X className="size-4" />
            </Button>
          </div>
        </div>

        <div className="flex items-center justify-end gap-2 px-6 pb-6">
          {actions || (
            <Button variant="outline" onClick={() => onOpenChange(false)}>
              Close
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}

export { ErrorDialog };
export type { ErrorDialogProps };

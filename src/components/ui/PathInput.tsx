import { useState } from 'react';
import { FolderOpen, FileText, X, Loader2 } from 'lucide-react';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';

export interface PathValidationResult {
  valid: boolean;
  exists: boolean;
  is_directory: boolean;
  hint: string;
}

interface PathInputProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  pathType: 'file' | 'directory';
  fileFilter?: string; // e.g., 'steamcmd.exe'
  placeholder?: string;
  hint?: string;
  disabled?: boolean;
  className?: string;
}

export function PathInput({
  label,
  value,
  onChange,
  pathType,
  fileFilter,
  placeholder,
  hint,
  disabled = false,
  className,
}: PathInputProps) {
  const [isValidating, setIsValidating] = useState(false);
  const [validationHint, setValidationHint] = useState<string | null>(null);

  const handleBrowse = async () => {
    try {
      let selected: string | null = null;

      if (pathType === 'directory') {
        selected = await open({
          directory: true,
          multiple: false,
          title: `Select ${label}`,
        });
      } else {
        selected = await open({
          directory: false,
          multiple: false,
          title: `Select ${label}`,
          filters: fileFilter
            ? [
                {
                  name: 'Executable',
                  extensions: [
                    fileFilter.includes('.') ? fileFilter.split('.').pop()! : fileFilter,
                  ],
                },
              ]
            : undefined,
        });
      }

      if (selected) {
        onChange(selected);
        // Validate the selected path
        await validatePath(selected);
      }
    } catch (e) {
      console.error('Browse dialog error:', e);
    }
  };

  const handleClear = () => {
    onChange('');
    setValidationHint(null);
  };

  const validatePath = async (path: string) => {
    if (!path.trim()) {
      setValidationHint(null);
      return;
    }

    setIsValidating(true);
    try {
      const pathTypeStr = pathType === 'directory' ? 'server_folder' : 'steamcmd';
      const result = await invoke<PathValidationResult>('validate_path', {
        path,
        pathType: pathTypeStr,
      });

      if (!result.valid) {
        setValidationHint(result.hint);
      } else {
        setValidationHint(result.hint);
      }
    } catch (e) {
      console.error('Validation error:', e);
      setValidationHint(null);
    } finally {
      setIsValidating(false);
    }
  };

  const handleBlur = async () => {
    if (value.trim()) {
      await validatePath(value);
    }
  };

  const isInvalid = !!validationHint && !validationHint.includes('auto-discovery');

  return (
    <div className={cn('space-y-1.5', className)}>
      <label className="block text-sm font-medium">{label}</label>

      <div className="flex gap-2">
        <div className="relative flex-1">
          <input
            type="text"
            value={value}
            onChange={(e) => onChange(e.target.value)}
            onBlur={handleBlur}
            placeholder={placeholder}
            disabled={disabled || isValidating}
            className={cn(
              'w-full h-9 rounded-lg border bg-background px-3 pr-10 text-sm transition-colors',
              'focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
              'placeholder:text-muted-foreground',
              'disabled:opacity-50 disabled:cursor-not-allowed',
              isInvalid ? 'border-destructive ring-1 ring-destructive' : 'border-input'
            )}
          />

          {/* Status icon */}
          <div className="absolute right-2 top-1/2 -translate-y-1/2">
            {isValidating ? (
              <Loader2 className="size-4 animate-spin text-muted-foreground" />
            ) : value && !isInvalid ? (
              <FileText className="size-4 text-primary" />
            ) : null}
          </div>
        </div>

        {/* Browse button */}
        <Button
          type="button"
          variant="outline"
          size="sm"
          onClick={handleBrowse}
          disabled={disabled}
          className="h-9 gap-1.5"
          title={`Browse for ${pathType === 'directory' ? 'folder' : 'file'}`}>
          <FolderOpen className="size-4" />
          Browse
        </Button>

        {/* Clear button */}
        {value && (
          <Button
            type="button"
            variant="ghost"
            size="icon-sm"
            onClick={handleClear}
            disabled={disabled}
            className="h-9 w-9"
            title="Clear and use auto-discovery">
            <X className="size-4" />
          </Button>
        )}
      </div>

      {/* Error or hint message */}
      {isInvalid ? (
        <p className="text-xs text-destructive" title={validationHint ?? undefined}>
          {validationHint}
        </p>
      ) : validationHint ? (
        <p className="text-xs text-muted-foreground" title={validationHint}>
          {validationHint}
        </p>
      ) : hint ? (
        <p className="text-xs text-muted-foreground">{hint}</p>
      ) : null}
    </div>
  );
}

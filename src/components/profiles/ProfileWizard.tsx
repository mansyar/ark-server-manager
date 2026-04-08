import { useState, useEffect } from 'react';
import { X, ChevronLeft, ChevronRight, Check, Map, Users, Lock, FolderOpen } from 'lucide-react';
import { useProfilesStore } from '@/stores/profilesStore';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { ARK_MAPS, type ArkMap } from '@/types/profile';
import {
  profileNameSchema,
  difficultySchema,
  maxPlayersSchema,
  adminPasswordSchema,
  portSchema,
  mapSchema,
} from '@/lib/validation';
import type { CreateProfileInput } from '@/lib/validation';
import { PROFILE_SCHEMA_VERSION } from '@/types/profile';
import type { Profile } from '@/types/profile';
import { WizardSecurityStep, WizardPathsStep, WizardReviewStep } from './WizardSteps';

interface FormData {
  name: string;
  map: ArkMap | '';
  difficulty: number;
  maxPlayers: number;
  adminPassword: string;
  adminPasswordConfirm: string;
  port: number;
  serverInstallPath: string;
  steamcmdPath: string;
}

interface FormErrors {
  name?: string[];
  map?: string[];
  difficulty?: string[];
  maxPlayers?: string[];
  adminPassword?: string[];
  port?: string[];
  adminPasswordConfirm?: string[];
  serverInstallPath?: string[];
  steamcmdPath?: string[];
}

const initialFormData: FormData = {
  name: '',
  map: '',
  difficulty: 5.0,
  maxPlayers: 70,
  adminPassword: '',
  adminPasswordConfirm: '',
  port: 27015,
  serverInstallPath: '',
  steamcmdPath: '',
};

function ProfileWizard() {
  const { wizardOpen, setWizardOpen, createProfile } = useProfilesStore();
  const [step, setStep] = useState(1);
  const [formData, setFormData] = useState<FormData>(initialFormData);
  const [errors, setErrors] = useState<FormErrors>({});
  const [showPassword, setShowPassword] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);

  // Reset form when wizard closes
  useEffect(() => {
    if (!wizardOpen) {
      setStep(1);
      setFormData(initialFormData);
      setErrors({});
      setShowPassword(false);
    }
  }, [wizardOpen]);

  // Close on escape
  useEffect(() => {
    if (!wizardOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        setWizardOpen(false);
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [wizardOpen, setWizardOpen]);

  const updateField = <K extends keyof FormData>(field: K, value: FormData[K]) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    // Clear error for this field
    if (errors[field as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [field]: undefined }));
    }
  };

  const validateStep = (currentStep: number): boolean => {
    const newErrors: FormErrors = {};

    if (currentStep === 1) {
      const result = profileNameSchema.safeParse(formData.name);
      if (!result.success) {
        newErrors.name = result.error.issues.map(
          (e: { message?: string }) => e.message ?? 'Invalid value'
        );
      }
    }

    if (currentStep === 2) {
      const difficultyResult = difficultySchema.safeParse(formData.difficulty);
      if (!difficultyResult.success) {
        newErrors.difficulty = difficultyResult.error.issues.map(
          (e: { message?: string }) => e.message ?? 'Invalid value'
        );
      }

      const maxPlayersResult = maxPlayersSchema.safeParse(formData.maxPlayers);
      if (!maxPlayersResult.success) {
        newErrors.maxPlayers = maxPlayersResult.error.issues.map(
          (e: { message?: string }) => e.message ?? 'Invalid value'
        );
      }

      const mapResult = mapSchema.safeParse(formData.map);
      if (!mapResult.success) {
        newErrors.map = [mapResult.error.issues[0]?.message ?? 'Invalid map'];
      }
    }

    if (currentStep === 3) {
      const passwordResult = adminPasswordSchema.safeParse(formData.adminPassword);
      if (!passwordResult.success) {
        newErrors.adminPassword = passwordResult.error.issues.map(
          (e: { message?: string }) => e.message ?? 'Invalid value'
        );
      }

      if (formData.adminPassword !== formData.adminPasswordConfirm) {
        newErrors.adminPasswordConfirm = ['Passwords do not match'];
      }

      const portResult = portSchema.safeParse(formData.port);
      if (!portResult.success) {
        newErrors.port = portResult.error.issues.map(
          (e: { message?: string }) => e.message ?? 'Invalid value'
        );
      }
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleNext = () => {
    if (validateStep(step)) {
      setStep((s) => Math.min(s + 1, 5));
    }
  };

  const handleBack = () => {
    setStep((s) => Math.max(s - 1, 1));
  };

  const handleSubmit = async () => {
    if (!validateStep(5)) return;

    setIsSubmitting(true);
    try {
      const input: CreateProfileInput = {
        name: formData.name,
        map: formData.map as ArkMap,
        difficulty: formData.difficulty,
        maxPlayers: formData.maxPlayers,
        adminPassword: formData.adminPassword,
        port: formData.port,
      };

      const profile: Profile = {
        schema_version: PROFILE_SCHEMA_VERSION,
        name: input.name,
        map: input.map,
        difficulty: input.difficulty,
        max_players: input.maxPlayers,
        admin_password: input.adminPassword,
        port: input.port,
        server_install_path: formData.serverInstallPath || null,
        steamcmd_path: formData.steamcmdPath || null,
        extra_settings: {},
        extra_user_settings: {},
      };

      await createProfile(profile);
    } catch (e) {
      console.error('Failed to create profile:', e);
      setErrors({ name: [String(e)] });
    } finally {
      setIsSubmitting(false);
    }
  };

  if (!wizardOpen) return null;

  const steps = [
    { number: 1, title: 'Name', icon: Map },
    { number: 2, title: 'Server', icon: Users },
    { number: 3, title: 'Security', icon: Lock },
    { number: 4, title: 'Paths', icon: FolderOpen },
    { number: 5, title: 'Review', icon: Check },
  ];

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/50 backdrop-blur-sm animate-in fade-in duration-200"
        onClick={() => setWizardOpen(false)}
        aria-hidden="true"
      />

      {/* Dialog */}
      <div
        className={cn(
          'relative z-10 w-full max-w-lg mx-4 rounded-xl bg-card text-card-foreground shadow-xl',
          'animate-in fade-in zoom-in-95 duration-200',
          'ring-1 ring-foreground/10 flex flex-col max-h-[90vh]'
        )}>
        {/* Header */}
        <div className="flex items-center justify-between p-6 pb-4 border-b">
          <div>
            <h2 className="text-lg font-semibold">Create New Profile</h2>
            <p className="text-sm text-muted-foreground mt-0.5">
              Step {step} of 5: {steps[step - 1].title}
            </p>
          </div>
          <Button
            variant="ghost"
            size="icon-xs"
            onClick={() => setWizardOpen(false)}
            aria-label="Close">
            <X className="size-4" />
          </Button>
        </div>

        {/* Progress */}
        <div className="flex items-center gap-2 px-6 py-4 border-b">
          {steps.map((s, i) => {
            const Icon = s.icon;
            const isActive = s.number === step;
            const isComplete = s.number < step;

            return (
              <div key={s.number} className="flex items-center flex-1">
                <div
                  className={cn(
                    'flex items-center gap-2 rounded-full px-3 py-1.5 text-xs font-medium transition-all',
                    isActive && 'bg-primary text-primary-foreground',
                    isComplete && 'bg-primary/10 text-primary',
                    !isActive && !isComplete && 'bg-muted text-muted-foreground'
                  )}>
                  {isComplete ? <Check className="size-3" /> : <Icon className="size-3" />}
                  <span className="hidden sm:inline">{s.title}</span>
                </div>
                {i < steps.length - 1 && (
                  <div
                    className={cn('flex-1 h-0.5 mx-2', isComplete ? 'bg-primary' : 'bg-muted')}
                  />
                )}
              </div>
            );
          })}
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6">
          {/* Step 1: Name */}
          {step === 1 && (
            <div className="space-y-4">
              <div>
                <label htmlFor="profile-name" className="block text-sm font-medium mb-1.5">
                  Profile Name
                </label>
                <input
                  id="profile-name"
                  type="text"
                  value={formData.name}
                  onChange={(e) => updateField('name', e.target.value)}
                  placeholder="My Awesome Server"
                  className={cn(
                    'w-full h-9 rounded-lg border bg-background px-3 text-sm transition-colors',
                    'focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
                    'placeholder:text-muted-foreground',
                    errors.name ? 'border-destructive ring-1 ring-destructive' : 'border-input'
                  )}
                  maxLength={64}
                  autoFocus
                />
                {errors.name && <p className="text-xs text-destructive mt-1">{errors.name[0]}</p>}
                <p className="text-xs text-muted-foreground mt-1">
                  1-64 characters. Avoid special characters: / \ : * ? " &lt; &gt; |
                </p>
              </div>
            </div>
          )}

          {/* Step 2: Server Settings */}
          {step === 2 && (
            <div className="space-y-6">
              {/* Map */}
              <div>
                <label htmlFor="map-select" className="block text-sm font-medium mb-1.5">
                  ARK Map
                </label>
                <select
                  id="map-select"
                  value={formData.map}
                  onChange={(e) => updateField('map', e.target.value as ArkMap)}
                  className={cn(
                    'w-full h-9 rounded-lg border bg-background px-3 text-sm transition-colors',
                    'focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2',
                    errors.map ? 'border-destructive ring-1 ring-destructive' : 'border-input'
                  )}>
                  <option value="">Select a map...</option>
                  {ARK_MAPS.map((map) => (
                    <option key={map} value={map}>
                      {map}
                    </option>
                  ))}
                </select>
                {errors.map && <p className="text-xs text-destructive mt-1">{errors.map[0]}</p>}
              </div>

              {/* Difficulty */}
              <div>
                <label htmlFor="difficulty" className="block text-sm font-medium mb-1.5">
                  Difficulty: {formData.difficulty.toFixed(1)}
                </label>
                <input
                  id="difficulty"
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
                  <p className="text-xs text-destructive mt-1">{errors.difficulty[0]}</p>
                )}
              </div>

              {/* Max Players */}
              <div>
                <label htmlFor="max-players" className="block text-sm font-medium mb-1.5">
                  Max Players: {formData.maxPlayers}
                </label>
                <input
                  id="max-players"
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
                  <p className="text-xs text-destructive mt-1">{errors.maxPlayers[0]}</p>
                )}
              </div>
            </div>
          )}

          {/* Step 3: Security */}
          {step === 3 && (
            <WizardSecurityStep
              adminPassword={formData.adminPassword}
              adminPasswordConfirm={formData.adminPasswordConfirm}
              port={formData.port}
              showPassword={showPassword}
              errors={errors}
              onAdminPasswordChange={(v) => updateField('adminPassword', v)}
              onAdminPasswordConfirmChange={(v) => updateField('adminPasswordConfirm', v)}
              onPortChange={(v) => updateField('port', v)}
              onShowPasswordToggle={() => setShowPassword(!showPassword)}
            />
          )}

          {/* Step 4: Install Paths */}
          {step === 4 && (
            <WizardPathsStep
              serverInstallPath={formData.serverInstallPath}
              steamcmdPath={formData.steamcmdPath}
              onServerInstallPathChange={(v) => updateField('serverInstallPath', v)}
              onSteamcmdPathChange={(v) => updateField('steamcmdPath', v)}
            />
          )}

          {/* Step 5: Review */}
          {step === 5 && (
            <WizardReviewStep
              name={formData.name}
              map={formData.map}
              difficulty={formData.difficulty}
              maxPlayers={formData.maxPlayers}
              port={formData.port}
            />
          )}
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between p-6 pt-4 border-t">
          <Button variant="outline" onClick={handleBack} disabled={step === 1}>
            <ChevronLeft className="size-4 mr-1" />
            Back
          </Button>

          {step < 5 ? (
            <Button onClick={handleNext}>
              Next
              <ChevronRight className="size-4 ml-1" />
            </Button>
          ) : (
            <Button onClick={handleSubmit} disabled={isSubmitting}>
              {isSubmitting ? 'Creating...' : 'Create Profile'}
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}

export { ProfileWizard };

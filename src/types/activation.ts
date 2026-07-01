export type ActivationStepKind = "application" | "browser";

export type ActivationStepStatus = "success" | "failure" | "warning" | "skipped";

export type ActivationStepSummary = {
  kind: ActivationStepKind;
  label: string;
  status: ActivationStepStatus;
  detail?: string | null;
  pid?: number | null;
};

export type ActivateSessionResult = {
  steps: ActivationStepSummary[];
  activationLogPath?: string | null;
};

export type ActivationPreviewStep = {
  stepType: string;
  label: string;
  argv: string[];
  cwd?: string | null;
  wouldSkip?: boolean | null;
  skipDetail?: string | null;
};

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

/** Payload bubbled from hub/editor to the shared activation overlay. */
export type ActivationCompletePayload = {
  label: string;
  result: ActivateSessionResult | null;
  error: string | null;
};

/** Dry-run preview payload (no processes spawned). */
export type PreviewCompletePayload = {
  label: string;
  steps: ActivationPreviewStep[] | null;
  error: string | null;
};

export type ActivationPreviewStep = {
  stepType: string;
  label: string;
  argv: string[];
  cwd?: string | null;
  wouldSkip?: boolean | null;
  skipDetail?: string | null;
};

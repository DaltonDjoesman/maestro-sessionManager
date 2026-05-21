export type ActivationStepKind = "application" | "browser";

export type ActivationStepStatus = "success" | "failure" | "warning" | "skipped";

export type ActivationStepSummary = {
  kind: ActivationStepKind;
  label: string;
  status: ActivationStepStatus;
  detail?: string | null;
  pid?: number | null;
};

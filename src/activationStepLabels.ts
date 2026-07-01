import type { ActivationStepKind } from "./types/activation";

const KIND_LABELS: Record<ActivationStepKind, string> = {
  browser: "Browser",
  application: "Application",
};

export function activationKindLabel(kind: ActivationStepKind): string {
  return KIND_LABELS[kind] ?? kind;
}

/** Human-readable row title: prefer executable label; fall back to kind. */
export function activationStepTitle(kind: ActivationStepKind, label: string): string {
  const trimmed = label.trim();
  if (trimmed.length > 0) return trimmed;
  return activationKindLabel(kind);
}

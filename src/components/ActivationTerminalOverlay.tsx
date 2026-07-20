import { useEffect, useId } from "react";
import { pt } from "../i18n/pt";
import type {
  ActivateSessionResult,
  ActivationPreviewStep,
  ActivationStepStatus,
  ActivationStepSummary,
} from "../types/activation";

export type ActivationOutcome = "success" | "caution" | "failure";

export type OverlayMode = "activation" | "preview";

type ActivationTerminalOverlayProps = {
  open: boolean;
  sessionLabel: string;
  mode?: OverlayMode;
  result: ActivateSessionResult | null;
  previewSteps?: ActivationPreviewStep[] | null;
  error: string | null;
  openLogError?: string | null;
  onClose: () => void;
  onOpenLog?: () => void;
};

export function deriveActivationOutcome(
  result: ActivateSessionResult | null,
  error: string | null,
): ActivationOutcome {
  if (error) return "failure";
  const steps = result?.steps ?? [];
  if (steps.some((s) => s.status === "failure")) return "failure";
  if (steps.some((s) => s.status === "warning" || s.status === "skipped")) return "caution";
  return "success";
}

function statusClass(status: ActivationStepStatus): string {
  switch (status) {
    case "success":
      return "log-success";
    case "failure":
      return "log-error";
    case "warning":
      return "log-warning";
    case "skipped":
      return "log-info";
    default:
      return "log-info";
  }
}

function statusLabel(status: ActivationStepStatus): string {
  switch (status) {
    case "success":
      return pt.activation.statusSuccess;
    case "failure":
      return pt.activation.statusFailure;
    case "warning":
      return pt.activation.statusWarning;
    case "skipped":
      return pt.activation.statusSkipped;
    default:
      return status;
  }
}

function outcomeTitle(outcome: ActivationOutcome): string {
  switch (outcome) {
    case "success":
      return pt.activation.outcomeSuccess;
    case "caution":
      return pt.activation.outcomeCaution;
    case "failure":
      return pt.activation.outcomeFailure;
  }
}

function summarizeSteps(steps: ActivationStepSummary[]): string {
  let ok = 0;
  let failed = 0;
  let warned = 0;
  let skipped = 0;
  for (const s of steps) {
    if (s.status === "success") ok += 1;
    else if (s.status === "failure") failed += 1;
    else if (s.status === "warning") warned += 1;
    else if (s.status === "skipped") skipped += 1;
  }
  return pt.activation.summaryCounts({ ok, failed, warned, skipped });
}

function formatArgv(argv: string[]): string {
  if (argv.length === 0) return "";
  return argv.map((a) => (/\s/.test(a) ? `"${a}"` : a)).join(" ");
}

export function ActivationTerminalOverlay({
  open,
  sessionLabel,
  mode = "activation",
  result,
  previewSteps = null,
  error,
  openLogError = null,
  onClose,
  onOpenLog,
}: ActivationTerminalOverlayProps) {
  const titleId = useId();
  const isPreview = mode === "preview";
  const steps = result?.steps ?? [];
  const preview = previewSteps ?? [];
  const logPath = result?.activationLogPath?.trim() || null;
  const outcome = isPreview ? "caution" : deriveActivationOutcome(result, error);
  const canOpenLog = Boolean(!isPreview && logPath && onOpenLog);
  const titlePrefix = isPreview ? pt.activation.previewTitle : pt.activation.title;

  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        onClose();
      }
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [open, onClose]);

  if (!open) return null;

  return (
    <div
      className="terminal-overlay"
      role="dialog"
      aria-modal="true"
      aria-labelledby={titleId}
      data-od-id="activation-terminal-overlay"
      data-mode={mode}
    >
      <div className="terminal-header">
        <div className={`terminal-title terminal-title--${outcome}`} id={titleId}>
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" aria-hidden>
            <path d="M4 17l6-6-6-6M12 19h10" />
          </svg>
          <span>
            {titlePrefix} — {sessionLabel}
          </span>
        </div>
        <div className="terminal-esc-hint">{pt.activation.escHint}</div>
      </div>

      <div className="terminal-body">
        {isPreview ? (
          <>
            <div className="terminal-outcome terminal-outcome--caution">
              {pt.activation.previewOutcome}
              {preview.length > 0 ? ` · ${preview.length} passo${preview.length === 1 ? "" : "s"}` : null}
            </div>
            <div className="terminal-log-line">
              <span className="log-timestamp">[·]</span>
              <span className="log-info">{pt.activation.previewBadge}</span>
            </div>
          </>
        ) : (
          <div className={`terminal-outcome terminal-outcome--${outcome}`}>
            {outcomeTitle(outcome)}
            {steps.length > 0 ? ` · ${summarizeSteps(steps)}` : null}
          </div>
        )}

        {error ? (
          <div className="terminal-log-line">
            <span className="log-timestamp">[!]</span>
            <span className="log-error">
              {isPreview ? pt.activation.previewError(error) : pt.activation.invokeError(error)}
            </span>
          </div>
        ) : null}

        {isPreview ? (
          <>
            {preview.length === 0 && !error ? (
              <div className="terminal-log-line">
                <span className="log-timestamp">[·]</span>
                <span className="log-info">{pt.activation.emptySteps}</span>
              </div>
            ) : null}
            {preview.map((step, index) => (
              <div key={`${step.label}-${index}`} className="terminal-log-line">
                <span className="log-timestamp">
                  [{step.wouldSkip ? pt.activation.statusWouldSkip : step.stepType}]
                </span>
                <span className={step.wouldSkip ? "log-info" : "log-success"}>
                  {step.label}
                  {formatArgv(step.argv) ? ` — ${formatArgv(step.argv)}` : ""}
                  {step.cwd?.trim() ? ` (cwd ${step.cwd.trim()})` : ""}
                  {step.skipDetail?.trim() ? ` — ${step.skipDetail.trim()}` : ""}
                </span>
              </div>
            ))}
          </>
        ) : (
          <>
            {steps.length === 0 && !error ? (
              <div className="terminal-log-line">
                <span className="log-timestamp">[·]</span>
                <span className="log-info">{pt.activation.emptySteps}</span>
              </div>
            ) : null}
            {steps.map((step, index) => (
              <div key={`${step.label}-${index}`} className="terminal-log-line">
                <span className="log-timestamp">[{statusLabel(step.status)}]</span>
                <span className={statusClass(step.status)}>
                  {step.label}
                  {step.detail?.trim() ? ` — ${step.detail.trim()}` : ""}
                  {step.pid != null ? ` (pid ${step.pid})` : ""}
                </span>
              </div>
            ))}
          </>
        )}

        {openLogError ? (
          <div className="terminal-log-line">
            <span className="log-timestamp">[!]</span>
            <span className="log-error">{openLogError}</span>
          </div>
        ) : null}
      </div>

      <div className="terminal-footer">
        <button type="button" className="btn" onClick={onClose} data-od-id="close-terminal-btn">
          {pt.activation.dismiss}
        </button>
        {canOpenLog ? (
          <button
            type="button"
            className="btn btn-primary"
            onClick={onOpenLog}
            data-od-id="open-log-btn"
          >
            {pt.activation.openLog}
          </button>
        ) : null}
      </div>
    </div>
  );
}

## Context

`activate_session_profile` already returns `ActivateSessionResult` (`steps[]` with status/label/detail/pid, optional `activationLogPath`). Hub and editor invoke it, call `onActivated(label)`, and discard the result. Spec `activation-results-ui` currently says in-app results UI is out of scope. The HTML prototype (`design/maestro-desktop-prototype.html`) and design-handoff planned `ActivationTerminalOverlay` with step timeline + log affordance. `tauri-plugin-opener` is already a dependency.

## Goals / Non-Goals

**Goals:**

- After every activation from hub or editor, show a dismissible overlay with overall outcome and per-step timeline.
- Surface invoke failures (exception before/without a result payload) in the same overlay or an equivalent error state—not only silent `setError` banners.
- “Abrir log” opens the activation log file when a path is returned.
- Visual language aligned with the prototype terminal overlay and existing OKLCH tokens / modal primitives.

**Non-Goals:**

- Dry-run / preview button UI (separate capability; leave `activation-preview` UI-optional).
- Streaming logs while processes spawn.
- In-app log text viewer.
- Changing spawn/skip/browser backend behavior.
- Session teardown / Desactivar semantics.

## Decisions

| Decision | Choice | Alternatives considered |
|----------|--------|-------------------------|
| State ownership | Lift activation result (and busy/error) to `App.tsx` (or a thin shell-level holder) so hub and editor share one overlay instance | Duplicate overlay in each page (harder dismiss/focus consistency) |
| Component | New `ActivationTerminalOverlay.tsx` controlled by props: `open`, `result`, `sessionLabel`, `error`, `onClose`, `onOpenLog` | Revive old generic modal table only |
| When to open | Always after invoke settles: success, mixed steps, or catch-path error | Only on failure (users still need confirmation that apps launched) |
| Overall outcome | Derive from steps: any `failure` → failed header; else any `warning`/`skipped` → caution; else success | Separate backend `overallStatus` field (nice-to-have; avoid API change) |
| Open log | `@tauri-apps/plugin-opener` `openPath` on `activationLogPath`; hide button if null/missing | In-app fetch of log contents |
| CSS | Port/adapt `.terminal-overlay` styles from prototype into `components.css` (keep if cleanup change would strip unused modal CSS—land styles with this feature) | Inline styles |
| i18n | All strings via `pt.ts` (title, close, open log, status labels, empty steps) | Hardcoded PT in JSX |
| Invoke errors | If `invoke` throws, show overlay with error message and empty/partial steps; still update sidebar active label only when activation actually succeeded | Keep banner-only errors |

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Cleanup change deletes modal/terminal CSS as orphans | Implement overlay styles in this change; if cleanup runs first, keep `.modal*` / add `.terminal-*` as used |
| Opener blocked for paths outside allowlist | Confirm capability/allowlist covers app data dir activation logs; degrade to showing path + copy or error toast |
| Overlay blocks UI after success (annoying) | Clear primary dismiss (Esc / Fechar); do not auto-close on timer |
| Partial success unclear | Status chips per step + summary line (“3 ok, 1 falhou”) |

## Migration Plan

- No data migration. Spec + README wording update on archive/apply.
- Rollback: remove overlay component and restore discard-result behavior; backend unchanged.

## Open Questions

- None blocking. Optional later: show dry-run preview in the same overlay shell—explicitly not this change.

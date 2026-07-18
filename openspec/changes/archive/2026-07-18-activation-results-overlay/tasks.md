## 1. Overlay component and styles

- [x] 1.1 Add `ActivationTerminalOverlay` component (open/close, session label, overall outcome, step list with status/label/detail, dismiss control)
- [x] 1.2 Port/adapt terminal overlay CSS from the design prototype into `components.css` (and keep modal/overlay primitives this UI needs)
- [x] 1.3 Add pt-PT strings in `pt.ts` for overlay title, dismiss, open log, status labels, and invoke-error copy

## 2. App wiring

- [x] 2.1 Lift activation result / error / open state to `App.tsx` (or equivalent shell holder) and render one shared overlay
- [x] 2.2 Update `SessionHubPage` activation to pass `ActivateSessionResult` (and invoke errors) up instead of discarding the result
- [x] 2.3 Update `ProfileEditorScreen` activation the same way
- [x] 2.4 Only mark sidebar “active session” when activation succeeds without a hard invoke failure (keep existing label behavior for successful runs)

## 3. Open log

- [x] 3.1 Wire “Abrir log” via `tauri-plugin-opener` when `activationLogPath` is present; hide/disable when absent
- [x] 3.2 Confirm opener allowlist/capabilities cover activation log paths under the app data dir; handle open failures with visible feedback

## 4. Docs and verification

- [x] 4.1 Update README release-scope note that currently says activation has no in-app results overlay
- [ ] 4.2 Manual smoke: activate from hub (success), force a failed step or bad executable, activate from editor, dismiss overlay, open log when path exists
- [x] 4.3 Run frontend build/typecheck; ensure no regression in activation invoke paths

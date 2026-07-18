## Why

Activation already returns structured per-step results, but the UI discards them—so failed launches are silent and users cannot trust “Ativar.” The design prototype and earlier handoff planned a terminal-style overlay; the main spec later marked that UI out of scope. Restoring a mandatory results overlay closes the largest trust gap before further commercial polish.

## What Changes

- Make an in-app **activation results overlay** required after hub and editor activation (success, partial failure, or hard invoke error surfaced clearly).
- Show a scannable **step timeline** (status, label, optional detail) driven by `ActivateSessionResult`.
- Offer **Abrir log** when `activationLogPath` is present (via existing `tauri-plugin-opener`), without building a full in-app log viewer.
- Style the overlay after the prototype terminal/results pattern (tokens + shared modal/overlay primitives).
- Update `activation-results-ui` so displaying results is **required**, not optional / deferred.
- Update README release-scope wording that currently says there is no in-app results overlay.
- **Not in scope:** dry-run / preview UI (`activation-preview` stays UI-optional), import/export modals, session teardown/deactivate, streaming live logs during spawn.

## Capabilities

### New Capabilities

<!-- None — restores deferred UI for an existing capability. -->

### Modified Capabilities

- `activation-results-ui`: Require the client to present an activation results overlay (timeline + dismiss + optional open log) whenever activation completes from hub or editor; keep backend structured result contract.

## Impact

- Frontend: new `ActivationTerminalOverlay` (or equivalent) component; wire `SessionHubPage` / `ProfileEditorScreen` / `App.tsx` to hold result state instead of discarding `ActivateSessionResult`; `src/i18n/pt.ts`; CSS for terminal overlay (prototype / `components.css`).
- Backend: no required API change if `ActivateSessionResult` already includes steps + `activation_log_path`; verify opener permission for log paths under the app data dir.
- Specs: delta for `activation-results-ui`; README release note.
- Parallel change `cleanup-css-dead-rust` may touch modal/CSS primitives—coordinate so overlay styles are not deleted as “orphans” before this UI lands (or re-add primitives in this change).

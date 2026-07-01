## 1. DTO and types (phase 1 foundation)

- [x] 1.1 Extend `RunningAppCandidate` in `capture/assistant.rs` with `kind`, `display_name`, `icon_name`, `desktop_workspace` (serde camelCase, optional fields with defaults).
- [x] 1.2 Mirror new fields in `src/types/capture.ts` and ensure `list_assistant_running_apps` IPC still deserializes for existing UI during incremental work.

## 2. Freedesktop classification (phase 1 backend)

- [x] 2.1 Add `capture/desktop_index.rs` (or equivalent) to scan and cache `.desktop` files from standard paths (`/usr/share/applications`, `~/.local/share/applications`).
- [x] 2.2 Implement executable/argv0 → desktop entry matching and populate `displayName` + `iconName` + `kind: app`.
- [x] 2.3 Assign `kind: process` for survivors that pass filters but have no desktop match.
- [x] 2.4 Add unit tests for classification fixtures (Obsidian flatpak path, generic `/usr/bin/foo`, unmatched script).

## 3. Expanded noise filters (phase 1 backend)

- [x] 3.1 Filter `obexd`, `oosplash` (when appropriate), `node`+`npm run`/`npx` in `assistant_background_noise` or denylist.
- [x] 3.2 Add regression tests mirroring user-reported cases (`npm run tauri dev`, `obexd`, LibreOffice splash).
- [x] 3.3 Run `cargo test` in `src-tauri` and confirm existing `assistant_filter_tests` still pass.

## 4. Assistant UI — apps first (phase 1 frontend)

- [x] 4.1 Rename section copy to “Running apps” (or PT equivalent per product language) and de-emphasize PID/cmdline in card layout.
- [x] 4.2 Show `displayName` as card title; render `iconName` placeholder or themed icon slot when present.
- [x] 4.3 Add “Show processes” toggle defaulting to off; when on, render `kind: process` cards in a separate subdued section.
- [x] 4.4 Keep “Add selected to draft” behavior unchanged for both kinds when visible.
- [x] 4.5 Manual smoke: refresh assistant on dev machine; confirm `node`/`obexd` hidden by default and Obsidian/LibreOffice show as apps.

## 5. Desktop workspace spike (phase 2 backend)

- [x] 5.1 Add `platform/workspace.rs` with `session_type()` from `XDG_SESSION_TYPE` and trait for workspace lookup by PID.
- [x] 5.2 Implement X11 path: PID → window → `_NET_WM_DESKTOP` (evaluate `wmctrl -l -p` subprocess vs `x11rb`; document choice in code comment).
- [x] 5.3 Spike Cosmic/Wayland: attempt compositor D-Bus or document “unsupported” with graceful null workspace.
- [x] 5.4 Wire `desktop_workspace` into `list_running_app_candidates` when lookup succeeds.
- [x] 5.5 Add tests or scripted fixtures for X11 workspace mapping where CI allows; document manual X11 verification in smoke checklist.

## 6. Assistant UI — workspace grouping (phase 2 frontend)

- [x] 6.1 Group cards under “Workspace N” headings when `desktopWorkspace` is present; ascending numeric order.
- [x] 6.2 Render “No workspace” section for candidates without `desktopWorkspace`.
- [x] 6.3 Fall back to flat A–Z by `displayName` when no candidate has workspace metadata.
- [x] 6.4 Update `docs/linux-desktop.md` with workspace detection limitations (X11 vs Wayland).

## 7. Verification and docs

- [x] 7.1 Extend `docs/smoke-test-checklist.md` with assistant classification and workspace grouping checks.
- [x] 7.2 Run full verification per `docs/backend-workflow.md` before marking change complete.

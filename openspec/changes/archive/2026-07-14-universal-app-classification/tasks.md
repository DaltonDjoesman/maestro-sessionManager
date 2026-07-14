## 1. Expanded Freedesktop index (phase A)

- [x] 1.1 Add standard Flatpak and Snap `.desktop` directories to `DesktopIndex::load()` when paths exist.
- [x] 1.2 Parse and index `TryExec` and `StartupWMClass` in addition to `Exec`.
- [x] 1.3 Implement robust `Exec` token extraction for `bwrap`, `flatpak-spawn`, and AppImage wrapper patterns.
- [x] 1.4 Add unit tests with fixture `.desktop` files (deb, flatpak bwrap, snap).

## 2. App classifier scoring (phase A)

- [x] 2.1 Create `capture/classifier.rs` with score constants and `classify(score_input) -> (kind, confidence)`.
- [x] 2.2 Wire classifier into assistant pipeline replacing direct `is_known_gui_basename` checks.
- [x] 2.3 Remove `is_known_gui_basename` and the hardcoded `KNOWN` basename list from `desktop_index.rs`.
- [x] 2.4 Add unit tests: window-only app, desktop-only tray daemon, flatpak `/app/` path, excluded noise.

## 3. DTO and API extensions (phase A)

- [x] 3.1 Add optional `windowTitle` and `classificationConfidence` to `RunningAppCandidate` (Rust + TypeScript).
- [x] 3.2 Ensure serde defaults keep backward compatibility for existing UI.

## 4. Window-anchored discovery (phase B)

- [x] 4.1 Create `capture/window_discovery.rs` building candidates from `WorkspaceIndex::windows()` on X11.
- [x] 4.2 Enrich each window row with process metadata (`sysinfo`) and `DesktopIndex` match.
- [x] 4.3 Emit separate rows per window when same PID spans multiple workspaces or distinct titles.
- [x] 4.4 Fall back to current process-first pipeline when window list is empty or session is unsupported.
- [x] 4.5 Update dedupe key to `(app_key, desktop_workspace, window_title_key)`.

## 5. Frontend polish (phase B)

- [x] 5.1 Show `windowTitle` as subtitle or secondary line when it differs from `displayName`.
- [x] 5.2 Optional subtle badge for `classificationConfidence: low` when shown in process toggle.
- [x] 5.3 Verify workspace grouping with multi-window browser smoke case.

## 6. Documentation and verification

- [x] 6.1 Update `docs/linux-desktop.md` with classification signal matrix and install-type notes.
- [x] 6.2 Extend `docs/smoke-test-checklist.md` for Flatpak/Snap/AppImage scenarios where available.
- [x] 6.3 Run `cd src-tauri && cargo test` and `npm run build` per backend workflow.

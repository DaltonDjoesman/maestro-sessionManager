## 1. Classifier Tier A (unblock Cosmic capture)

- [ ] 1.1 Update `app-classifier-scoring` unit tests: keep “desktop without window → process” when windows are enumerable; add “strong desktop match on Wayland without window list → app”
- [ ] 1.2 Implement session-/source-aware no-window penalty in `capture/classifier.rs` (and call sites in `window_discovery.rs`) so process fallback on empty window index meets `APP_THRESHOLD` for strong `.desktop` matches
- [ ] 1.3 Run `cargo test` for classifier + window discovery modules; confirm X11-oriented cases still pass

## 2. Window source merge (X11 + supplemental XWayland)

- [ ] 2.1 Introduce a small `WindowSource` / merge helper used by `WorkspaceIndex::load()` (keep existing `WindowRecord` shape)
- [ ] 2.2 Allow `wmctrl -l -p` on Wayland as a supplemental XWayland source when it succeeds (do not treat as complete coverage)
- [ ] 2.3 Wire `discover_running_app_candidates` to use window-first whenever the merged index is non-empty, regardless of `x11` vs `wayland` session string
- [ ] 2.4 Add/extend unit tests for wmctrl parsing merge and empty-vs-non-empty source selection

## 3. Wayland foreign-toplevel adapter (Tier B)

- [ ] 3.1 Spike on Cosmic: can an unprivileged process bind `ext-foreign-toplevel-list-v1` and read titles/`app_id`? Document result in `docs/linux-desktop.md`
- [ ] 3.2 Add Linux module for Wayland window snapshot (short-lived client or helper) mapping into `WindowRecord` (title, optional PID, optional workspace)
- [ ] 3.3 Optionally attach Cosmic `zcosmic_toplevel_info_v1` workspace metadata when available; soft-fail if denied
- [ ] 3.4 Register the adapter in `WorkspaceIndex::load()` merge; ensure bind/timeout failure yields empty Wayland source without failing the Tauri command
- [ ] 3.5 Add tests with mocked/fixture records for Wayland-sourced windows (no live compositor required in CI)

## 4. Docs and manual verification

- [ ] 4.1 Update `docs/linux-desktop.md` tables for Wayland/Cosmic capture, XWayland supplemental `wmctrl`, and protocol privilege notes
- [ ] 4.2 Manual check on Cosmic: native Wayland apps appear as `app` after refresh; XWayland app (e.g. Slack) appears; X11 session (if available) unchanged
- [ ] 4.3 Manual check: with foreign-toplevel working, multi-window / workspace grouping; without it, flat list still usable via Tier A

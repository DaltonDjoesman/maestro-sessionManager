## 1. Dependencies and unknown-workspace model

- [x] 1.1 Confirm `cosmic-protocols` (client) license fit; add it to `src-tauri/Cargo.toml` or fall back to vendored XML + `wayland-scanner` codegen per design
- [x] 1.2 Change Wayland window records so unresolved workspace is expressible (e.g. `WindowRecord.desktop: Option<u32>` or equivalent) without breaking `wmctrl` rows that always have an index
- [x] 1.3 Stop foreign-toplevel `unwrap_or(0)` from becoming candidate `desktopWorkspace: Some(0)` when Cosmic attach did not resolve a workspace

## 2. Cosmic workspace mapping

- [x] 2.1 Implement handle→stable 0-based index map from Cosmic/`ext` workspace manager events (fixture-friendly pure helpers)
- [x] 2.2 Bind `zcosmic_toplevel_info_v1` in the existing short-lived Wayland snapshot; collect workspace_enter/leave (and/or `ext_workspace_*`) per toplevel
- [x] 2.3 Match Cosmic toplevels to foreign-toplevel `WindowRecord`s (prefer protocol association; title/`app_id` fallback within the snapshot)
- [x] 2.4 Replace `try_attach_cosmic_workspace_metadata` soft-noop with real attachment; soft-fail (log + leave workspace unset) on bind/timeout/denial

## 3. Discovery plumbing and regression safety

- [x] 3.1 Ensure window-first discovery / merge paths propagate `Option` workspace correctly (omit `desktopWorkspace` when unset; keep X11/`wmctrl` indices intact)
- [x] 3.2 Update existing Wayland unit tests for soft-fail and multi-workspace fixtures; add mapping tests that do not require a live compositor
- [x] 3.3 Run `cargo test` for affected `src-tauri` modules and fix regressions

## 4. Docs and manual Cosmic check

- [x] 4.1 Update `docs/linux-desktop.md`: Cosmic workspace grouping via toplevel-info + handle→index; soft-fail omits workspace (no fake “Workspace 1”)
- [x] 4.2 Document manual verification: apps on Cosmic workspace 1 and 2 → distinct assistant headings after refresh

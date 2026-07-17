## Why

Wayland app discovery already lists Cosmic toplevels (titles/`app_id`), but Cosmic workspace index attachment was deferred as a soft-noop. Foreign-toplevel rows therefore keep `desktop = 0`, so the assistant always shows a single “Workspace 1” bucket instead of grouping windows by real Cosmic workspaces the way X11/`wmctrl` does.

## What Changes

- Wire Cosmic workspace metadata into the existing short-lived Wayland snapshot: bind `zcosmic_toplevel_info_v1` (and Cosmic/ext workspace manager as needed), map workspace handles to stable 0-based indices, and set `WindowRecord.desktop` / candidate `desktopWorkspace` per window.
- Keep soft-fail behavior when Cosmic protocols are missing, denied, or time out — discovery and capture stay usable; do **not** pretend every window is on workspace 0.
- Leave the X11/`wmctrl` path and IPC/UI contract unchanged; existing workspace grouping UI works once indices are real.
- Update `docs/linux-desktop.md` to describe Cosmic workspace grouping and how to verify it manually.
- Add fixture-based unit tests for handle→index mapping (no live compositor required in CI).
- No **BREAKING** API changes.

## Capabilities

### New Capabilities

<!-- None — this completes deferred Cosmic workspace behavior under existing capabilities. -->

### Modified Capabilities

- `desktop-workspace-hints`: Cosmic Wayland sessions MUST attach real per-window workspace indices when Cosmic toplevel/workspace protocols succeed; unresolved workspace MUST omit `desktopWorkspace` (not fake index `0`).
- `wayland-window-discovery`: Foreign-toplevel enumeration SHALL enrich records with Cosmic workspace indices via `zcosmic_toplevel_info_v1` + workspace-handle mapping when available, still soft-failing without failing the assistant.

## Impact

- **Rust**: `src-tauri/src/platform/wayland_windows.rs` (`try_attach_cosmic_workspace_metadata` and related snapshot state), possibly small helpers in `workspace.rs` / discovery for “unknown vs 0”; tests under the same modules.
- **Dependencies**: prefer `cosmic-protocols` (client feature) or equivalent thin generated bindings on top of existing `wayland-client` / `wayland-protocols` — avoid pulling full `libcosmic`.
- **Docs**: `docs/linux-desktop.md` Cosmic section.
- **Frontend**: no contract change; workspace headings light up once candidates carry distinct `desktopWorkspace` values.
- **Environments**: Pop!_OS Cosmic Wayland (primary); other compositors unchanged (empty Cosmic attach → omit workspace); X11/`wmctrl` unchanged.

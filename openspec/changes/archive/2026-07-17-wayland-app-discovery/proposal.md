## Why

Pop!_OS Cosmic (and other modern Linux desktops) now run as pure Wayland. Maestro’s running-apps assistant still gates window enumeration on `XDG_SESSION_TYPE=x11`/`tty` and then demotes strong `.desktop` matches without a mapped window to `kind: process`. The UI only shows `app` rows, so capture appears empty after the OS upgrade even though processes are found.

## What Changes

- Add a **Wayland session window-discovery path** (Cosmic first) so mapped top-level surfaces can drive window-first capture the same way `wmctrl` does on X11.
- Keep X11/`wmctrl` as the primary path when the session is X11; merge or prefer the best available window source per session.
- Adjust **classifier scoring** so that on Wayland, when no window list is available yet, a strong Freedesktop `.desktop` match still reaches the `app` threshold (without reopening the door to background daemons that fail noise filters).
- Extend **workspace hints** when the compositor exposes workspace/index metadata; otherwise keep the flat UI layout.
- Update `docs/linux-desktop.md` with Wayland/Cosmic capture expectations and manual verification steps.
- No **BREAKING** API changes to the Tauri command shape (`list_assistant_running_apps` / `RunningAppCandidate`); behavior and field population improve.

## Capabilities

### New Capabilities

- `wayland-window-discovery`: Discover mapped top-level windows (and optional workspace indices) on Wayland sessions via compositor-specific adapters, starting with Cosmic, with graceful no-op when unsupported.

### Modified Capabilities

- `window-anchored-discovery`: Window-first discovery MUST also apply when a Wayland window source returns mapped surfaces, not only when `wmctrl` succeeds on X11.
- `app-classifier-scoring`: Strong `.desktop` matches on Wayland without a window signal MUST still classify as `app` when noise filters pass; the “desktop file but no window → process” rule remains for cases where a window source is available and the process has no mapped surface.
- `desktop-workspace-hints`: When a supported Wayland compositor reports workspace metadata, attach `desktopWorkspace`; Cosmic is the reference target.

## Impact

- **Rust**: `src-tauri/src/platform/workspace.rs`, new Wayland/Cosmic adapter module(s), `capture/window_discovery.rs`, `capture/classifier.rs` (+ tests).
- **Docs**: `docs/linux-desktop.md`.
- **Frontend**: no required UI contract change; existing `filterByKind` / workspace grouping continue to work once candidates are `app` and optionally carry `desktopWorkspace`.
- **Dependencies**: possibly D-Bus (`zbus` or subprocess `busctl`/`gdbus`) for Cosmic; avoid pulling heavy X11 crates. External `wmctrl` remains X11-only.
- **Environments**: Pop!_OS Cosmic Wayland (primary), other Wayland desktops degrade to process+desktop until adapters exist; X11 behavior must stay intact.

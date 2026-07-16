# Maestro on Linux: `.desktop` entry and “Open with” (phase 2)

This document is for **maintainers** packaging Maestro for Linux desktops (GNOME, KDE, etc.). End users get the same UX whether or not file associations are registered.

## Application identity

The Tauri bundle identifier is:

- **`com.maestro.app`** (see `src-tauri/tauri.conf.json` → `identifier`)

Use the same string for **`StartupWMClass`** in the `.desktop` file so the window manager groups windows correctly once you confirm the actual WM class from a running build (`xprop WM_CLASS` on the window).

## Example `.desktop` file

Install to `/usr/share/applications/maestro.desktop` (or your distro’s equivalent):

```ini
[Desktop Entry]
Type=Application
Name=Maestro
Comment=Linux session environment manager
Exec=/usr/bin/maestro %U
Icon=maestro
Terminal=false
Categories=Utility;Development;
StartupWMClass=com.maestro.app
MimeType=x-scheme-handler/maestro;
```

Notes:

- Replace **`Exec`** with the real install path from your package (e.g. `/opt/Maestro/maestro` or the path inside an AppImage wrapper script).
- **`%f`** is appropriate for a single local file; **`%U`** allows URLs if you add URL handlers later.
- **`Icon=`** must match an installed Freedesktop icon name or a full path to a PNG/SVG.

## Optional MIME / “Open with Maestro” (post-MVP)

1. Define a private MIME type (e.g. `application/x-maestro-session`) and install a small XML under `/usr/share/mime/packages/`.
2. Run `update-mime-database` / `gtk-update-icon-cache` as required by the distro.
3. Point **`MimeType=`** at that MIME type and register Maestro as the default handler for it.
4. Use **`%f`** in `Exec` when the handler must receive a single file path.

Keep this **opt-in** in installers: missing MIME registration must not break launching Maestro from the app menu.

## Desktop workspace hints (profile assistant)

The running-apps assistant can **read** (not control) which virtual workspace a window belongs to, to group cards in the profile editor.

| Session | Mechanism | Expectation |
|---------|-----------|-------------|
| **X11** | `wmctrl -l -p` → EWMH desktop index per PID | Works when `wmctrl` is installed and the app has an X11 window |
| **Wayland (Cosmic)** | `ext-foreign-toplevel-list-v1` for titles/`app_id`; optional Cosmic `zcosmic_toplevel_info_v1` for workspace (soft-fail today → flat list) | Window-first discovery when the protocol binds; Tier A process+`.desktop` if not |
| **Wayland + XWayland** | Supplemental `wmctrl -l -p` for XWayland clients only | Partial coverage — never treated as complete alone |
| **Hybrid** (e.g. Flatpak app with `--ozone-platform=x11`) | Same as XWayland/`wmctrl` for that window’s PID | Best-effort per process |

Maestro **does not** move or focus windows between workspaces.

### Cosmic spike (`ext-foreign-toplevel-list-v1`) — 2026-07-16

On Pop!_OS Cosmic (`XDG_SESSION_TYPE=wayland`, `XDG_CURRENT_DESKTOP=COSMIC`), an **unprivileged** short-lived Wayland client **can** bind `ext_foreign_toplevel_list_v1` and receive `title` / `app_id` for mapped toplevels (verified live: Cursor, Vivaldi, TickTick, Slack, Maestro, etc.). Cosmic also advertises `zcosmic_toplevel_info_v1`; workspace **index** attachment via that protocol is deferred (soft-fail) — handle→index mapping needs Cosmic workspace protocol wiring. Until then, `desktopWorkspace` may be omitted and the UI stays a flat list while still listing `app` rows.

Manual check (X11): run `echo $XDG_SESSION_TYPE`, open apps on workspace 1 and 2, refresh the assistant, confirm section headings “Workspace 1” / “Workspace 2”. Multi-window browsers (e.g. Vivaldi) SHOULD appear as separate rows per window when titles or workspaces differ.

Manual check (Cosmic Wayland): refresh capture with native Wayland apps open; confirm `kind: app` rows. With an XWayland app (e.g. Slack) open, confirm it appears via supplemental `wmctrl` and/or foreign-toplevel. Without foreign-toplevel access, Tier A scoring still surfaces strong `.desktop` matches as `app`.

## App classification (running-apps assistant)

The assistant classifies each candidate as **`app`** or **`process`** using a scored model — not a hardcoded basename allowlist.

| Signal | Points | Notes |
|--------|--------|-------|
| Mapped top-level window (X11 / `wmctrl` / Wayland foreign-toplevel) | +100 | Window-first whenever the merged window index is non-empty |
| Matched `.desktop` entry (`Exec`, `TryExec`, `StartupWMClass`) | +80 | Index includes deb, Flatpak exports, Snap paths |
| `StartupWMClass` / title heuristic match | +60 | Used when PID is invalid but title matches |
| Flatpak `/app/` or Snap path segment | +20 | Install-type hint only |
| No-window penalty | −50 | Applied only when a window source returned mapped windows and this process has none |

**Threshold:** score ≥ **80** → `kind: app`; below → `process` (visible when “Show processes” is on).

On **Wayland without any window list**, strong `.desktop` matches that pass noise filters are **not** given the no-window penalty, so they still reach the app threshold (Tier A).

**Confidence:** `high` (window + desktop), `medium` (window only), `low` (neither strong signal). The UI may show a subtle **low** badge on process rows when the process toggle is enabled.

**Noise exclusion** (never listed): Chromium `--type=` subprocesses, `obexd`, LibreOffice `oosplash`, `node`/`npm run` dev tooling, and other infrastructure daemons — see `capture/assistant.rs` filters.

**Install types:** deb packages use `/usr/share/applications`; Flatpak adds `~/.local/share/flatpak/exports/share/applications` and `/var/lib/flatpak/exports/share/applications`; Snap adds `/var/lib/snapd/desktop/applications`. Flatpak `Exec` lines wrapped in `bwrap` are parsed for `/app/<id>` match keys.

## Verification checklist

- [ ] App menu entry launches Maestro.
- [ ] `StartupWMClass` matches the real window (adjust after `xprop` if needed).
- [ ] `Exec` works with spaces in install paths (quote if you wrap in a script).

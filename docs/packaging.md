# Packaging Maestro for Linux

Maintainer guide for building installable artifacts and native dependency families.
End users can also build from source with the [Tauri Linux prerequisites](https://tauri.app/start/prerequisites/).

## Bundle targets

`src-tauri/tauri.conf.json` sets:

```json
"bundle": {
  "active": true,
  "targets": ["deb", "appimage"]
}
```

On a Debian/Ubuntu-family host with Tauri Linux deps installed:

```bash
npm install
npm run tauri build
```

Artifacts typically land under `src-tauri/target/release/bundle/`:

| Target | Path (typical) | Notes |
|--------|----------------|-------|
| **`.deb`** | `deb/*.deb` | Installable on Debian/Ubuntu/Pop!_OS |
| **AppImage** | `appimage/*.AppImage` | Portable; needs FUSE on some hosts |

### Host limitations

- Building **`.deb` / AppImage** requires a Linux host with the matching toolchain (cross-building from macOS/Windows is not supported by this project’s default flow).
- AppImage generation may need `linuxdeploy` / FUSE; if AppImage fails on the builder, ship `.deb` and document the AppImage gap.
- `targets: "all"` previously pulled every Tauri platform bundle; we pin **`deb` + `appimage`** so CI/local builds stay focused on Linux portfolio artifacts.

Also see [linux-desktop.md](./linux-desktop.md) for `.desktop` / MIME notes after install.

After a successful local build, publish a tagged GitHub Release with the installers — see [releasing.md](./releasing.md).

## Native dependency families

### Debian / Ubuntu / Pop!_OS

Install Tauri’s documented packages (names evolve — verify against current Tauri docs), commonly including:

- `libwebkit2gtk-4.1-dev` (or the WebKitGTK version Tauri 2 requires)
- `libayatana-appindicator3-dev`
- `librsvg2-dev`
- `patchelf`
- build essentials: `build-essential`, `curl`, `wget`, `file`, `libssl-dev`

Optional for workspace discovery on X11/XWayland: **`wmctrl`**.

### Fedora (hints)

Rough equivalents (adjust versions to your release):

- `webkit2gtk4.1-devel`
- `libayatana-appindicator-gtk3-devel` (or distro AppIndicator package)
- `librsvg2-devel`
- `openssl-devel`
- `gcc`, `gcc-c++`, `make`, `curl`, `wget`, `file`, `patchelf`

### Arch Linux (hints)

- `webkit2gtk-4.1`
- `libayatana-appindicator`
- `librsvg`
- `openssl`
- base-devel group, `curl`, `wget`, `file`, `patchelf`

## Workspace grouping honesty

Maestro is a **session launcher**. Capture may group windows by desktop workspace when the compositor exposes indices (X11/`wmctrl`, Cosmic Wayland enrichment). On unsupported Wayland compositors:

- Listing apps still works (process + `.desktop` scoring; foreign-toplevel titles when available).
- Workspace headings are **omitted** — the UI shows a notice rather than inventing a fake “Workspace 1”.

See [linux-desktop.md](./linux-desktop.md) for compositor adapter status.

# Releasing Maestro

Maintainer guide for tagging a GitHub Release with Linux installers. Version string lives in:

- `package.json` → `version`
- `src-tauri/Cargo.toml` → `version`
- `src-tauri/tauri.conf.json` → `version`

Keep those three in sync before tagging.

CI (`.github/workflows/ci.yml`) builds the frontend and runs tests; it does **not** run `tauri build`. Releases are created manually from a Linux host with [Tauri prerequisites](https://tauri.app/start/prerequisites/).

Bundle targets and distro deps: [packaging.md](./packaging.md). Desktop entry notes after install: [linux-desktop.md](./linux-desktop.md).

## Pre-flight

From the repository root on Pop!_OS / Ubuntu-class Linux:

```bash
npm ci
npm run build
npm test
cd src-tauri && cargo test && cd ..
```

Optional but recommended before a public tag: walk [smoke-test-checklist.md](./smoke-test-checklist.md) against a release build once it exists.

## Build artifacts

```bash
npm run tauri build
```

Typical outputs:

| Target | Path |
|--------|------|
| `.deb` | `src-tauri/target/release/bundle/deb/*.deb` |
| AppImage | `src-tauri/target/release/bundle/appimage/*.AppImage` |

If AppImage generation fails (FUSE / `linuxdeploy`), ship the `.deb` only and note the gap in the release notes.

## Tag and GitHub Release

Example for **v0.1.0** (adjust version and notes path for later releases):

```bash
git tag -a v0.1.0 -m "First public release — Linux session launcher"
git push origin v0.1.0

gh release create v0.1.0 \
  --title "v0.1.0" \
  --notes-file docs/release-notes/v0.1.0.md \
  src-tauri/target/release/bundle/deb/*.deb \
  src-tauri/target/release/bundle/appimage/*.AppImage
```

Omit the AppImage glob if that artifact is missing. Release notes for the first public cut: [release-notes/v0.1.0.md](./release-notes/v0.1.0.md).

## After publishing

1. Confirm the [Releases](https://github.com/DaltonDjoesman/maestro-sessionManager/releases) page lists at least one installable file.
2. Confirm the README **Download** line points at that page.
3. For the next version, bump the three version fields, add `docs/release-notes/vX.Y.Z.md`, then repeat.

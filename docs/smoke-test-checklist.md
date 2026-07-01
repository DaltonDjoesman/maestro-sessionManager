# Maestro smoke test checklist (manual)

Use this after `npm run tauri dev` or a release build on **Pop!_OS** / Ubuntu-class Linux with [Tauri prerequisites](https://tauri.app/start/prerequisites/) installed.

## 1. App starts

- [ ] `npm install` then `npm run tauri dev` opens the Maestro window without Rust panic in the terminal.

## 2. Settings

- [ ] **Settings** opens from the home screen.
- [ ] Profiles root path validates (existing directory, writable).
- [ ] Save persists after restart (optional quick check).

## 3. Profiles catalog

- [ ] **Profiles** lists JSON files under the configured root.
- [ ] **New profile** creates a file and opens the editor.
- [ ] **Duplicate** creates a second file with “(copy)” in the name.
- [ ] **Delete** removes a profile after confirmation (use a disposable test file).

## 4. Profile editor and activation (mock browser)

- [ ] Copy [`docs/examples/smoke-session.profile.json`](examples/smoke-session.profile.json) into your profiles directory (same basename or rename to `*.json`).
- [ ] **Edit** loads the profile; **Save** works without validation error.
- [ ] **Activate session** runs without a pre-spawn validation error.
- [ ] Activation table shows at least one **browser** step and one **application** step; statuses are `success` or acceptable `warning` rows (warnings are OK for isolation hints).
- [ ] Invalid profile (e.g. empty `applications[].executable`) shows an error and does not claim success.

## 5. Rust tests (CI-friendly)

- [ ] `cd src-tauri && cargo test` passes.

## 6. Running-apps assistant (profile editor)

Requires **Settings → Show running-apps assistant** enabled.

- [ ] **Edit** a profile → **Refresh list** loads candidates without error.
- [ ] Default view shows **apps only** (e.g. Obsidian, LibreOffice main window) — not `node`/`npm run …`, `obexd`, or LibreOffice `oosplash`.
- [ ] **Show processes** reveals background programs in a separate subdued section; rows with weak signals may show a **low** confidence badge.
- [ ] Card titles use human **display names**; when the window title differs, it appears as a subtitle under the display name.
- [ ] PID and command line are under **Technical details** (window title also listed when present).
- [ ] **Add selected to draft** still appends launch rows and **Save** persists.
- [ ] On **X11** with `wmctrl` installed: cards group under **Workspace N** when windows span workspaces (optional; skip on pure Wayland).
- [ ] Multi-window browser (e.g. Vivaldi with tabs on different workspaces): refresh shows **separate cards** per window/workspace, not one collapsed row (optional).
- [ ] **Flatpak / Snap / AppImage** (when installed): running instance matches without manual basename allowlisting — display name from `.desktop` `Name` (optional; use whatever install types you have).

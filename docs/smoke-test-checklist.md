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

## 5. Cleanup divergences (optional)

- [ ] On **Home**, under **Context cleanup**, enter the profile path (relative to profiles root, e.g. `smoke-session.profile.json`) and **Scan divergences** returns a table (possibly empty).
- [ ] Do **not** terminate random PIDs on a production machine; skip or use a disposable test profile only.

## 6. Rust tests (CI-friendly)

- [ ] `cd src-tauri && cargo test` passes.

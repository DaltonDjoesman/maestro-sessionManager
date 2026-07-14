# Maestro smoke test checklist (manual)

Use this after `npm run tauri dev` or a release build on **Pop!_OS** / Ubuntu-class Linux with [Tauri prerequisites](https://tauri.app/start/prerequisites/) installed.

Scope note: activation runs without an in-app results overlay; dry-run and Advanced editor tab are out of scope for this release.

## 1. App starts

- [ ] `npm install` then `npm run tauri dev` opens the Maestro window without Rust panic in the terminal.
- [ ] A **sidebar** shows **Sessões**, **Captura**, **Definições** (and **Editor** while editing); **Sessões** is the default view (session hub).
- [ ] Window **header** shows Maestro logo/title and **theme toggle** (persists via Definições).

## 1b. Visual QA (design handoff)

Compare against `design/maestro-desktop-prototype.html` (inner app window only, pt-PT copy):

- [ ] **1440×900** — hub cards, sidebar, header match prototype layout.
- [ ] **1366×768**, **1024×768** — no horizontal scroll; drawer sidebar below 900px width.
- [ ] **390×844**, **360×800** — mobile drawer navigation usable.

## 2. Settings

- [ ] **Definições** opens from the sidebar.
- [ ] Settings are grouped (Geral, Sessões, Browser, Avançado); **Sobre** appears at the bottom with version (no separate About tab).
- [ ] Profiles root path validates (existing directory, writable).
- [ ] Theme (system/light/dark) updates the app chrome without restart.
- [ ] Save persists after restart (optional quick check).

## 3. Session hub

- [ ] Hub lists profiles as **grid cards** (prototype `profile-card` layout).
- [ ] Toolbar: search, **Importar** (file picker + name prompt), **Nova sessão**.
- [ ] **Ativar** on a valid card runs activation (no results overlay required).
- [ ] Clicking card body opens the profile editor.
- [ ] Overflow menu (**···**): duplicate, export, delete.
- [ ] Search filters by name; pins appear in **Fixadas**; last session appears under **Continuar** when valid.

## 4. Profile editor and activation (mock browser)

- [ ] Copy [`docs/examples/smoke-session.profile.json`](docs/examples/smoke-session.profile.json) into your profiles directory.
- [ ] Editor has sticky header (**Guardar**, **Ativar**) and tabs **Conteúdo** / **Captura**.
- [ ] **Guardar** works without validation error.
- [ ] **Ativar sessão** invokes activation (apps/browser launch as configured).
- [ ] Invalid profile shows an error and does not claim success.

## 5. Rust tests (CI-friendly)

- [ ] `cd src-tauri && cargo test` passes.

## 6. Running-apps assistant (Captura)

Capture is always available (no settings toggle).

- [ ] Sidebar **Captura** opens standalone capture screen (search, create profile from selection).
- [ ] **Captura** tab in editor works for contextual draft import.
- [ ] Refresh icon loads candidates without error.
- [ ] Default view shows **apps only** — not `node`/`npm run …`, `obexd`, or LibreOffice `oosplash`.
- [ ] **Adicionar selecionadas** (editor tab) appends launch rows; **Guardar** persists.
- [ ] Sidebar widget shows **sessão activa** after successful activation; **Desactivar** clears label.

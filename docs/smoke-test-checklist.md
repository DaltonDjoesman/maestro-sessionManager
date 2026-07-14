# Maestro smoke test checklist (manual)

Use this after `npm run tauri dev` or a release build on **Pop!_OS** / Ubuntu-class Linux with [Tauri prerequisites](https://tauri.app/start/prerequisites/) installed.

## 1. App starts

- [ ] `npm install` then `npm run tauri dev` opens the Maestro window without Rust panic in the terminal.
- [ ] A **sidebar** shows **Sessões**, **Captura**, **Definições** (and **Editor** while editing); **Sessões** is the default view (session hub).
- [ ] Window **header** shows Maestro title, decorative controls, and **theme toggle** (persists via Definições).

## 1b. Visual QA (design handoff)

Compare against `design/maestro-desktop-prototype.html` (inner app window only, pt-PT copy):

- [ ] **1440×900** — hub cards, sidebar, header match prototype layout.
- [ ] **1366×768**, **1024×768** — no horizontal scroll; drawer sidebar below 900px width.
- [ ] **390×844**, **360×800** — mobile drawer navigation usable.

## 2. Settings

- [ ] **Definições** opens from the sidebar.
- [ ] Settings are grouped (Geral, Sessões, Browser, Assistente, Avançado); **Sobre** appears at the bottom with version (no separate About tab).
- [ ] Profiles root path validates (existing directory, writable).
- [ ] Theme (system/light/dark) updates the app chrome without restart.
- [ ] Save persists after restart (optional quick check).

## 3. Session hub

- [ ] Hub lists profiles as **grid cards** (prototype `profile-card` layout).
- [ ] Toolbar: search, **Importar** (modal), **Nova sessão**.
- [ ] **Ativar** on a valid card runs activation and opens the **terminal overlay** (timeline/logs).
- [ ] **Editar** opens the profile editor.
- [ ] Overflow menu (**···**): duplicate, export, delete, dry-run.
- [ ] Search filters by name; pins appear in **Fixadas**; last session appears under **Continuar** when valid.

## 4. Profile editor and activation (mock browser)

- [ ] Copy [`docs/examples/smoke-session.profile.json`](examples/smoke-session.profile.json) into your profiles directory.
- [ ] Editor has sticky header (**Guardar**, **Ativar**) and tabs **Conteúdo** / **Captura** (if assistant on) / **Avançado**.
- [ ] **Guardar** works without validation error.
- [ ] **Ativar sessão** opens the shared **terminal overlay** with browser and application steps.
- [ ] **Dry-run** from **Avançado** or hub overflow shows argv in terminal overlay with **DRY-RUN** badge.
- [ ] Invalid profile shows an error and does not claim success.

## 5. Rust tests (CI-friendly)

- [ ] `cd src-tauri && cargo test` passes.

## 6. Running-apps assistant (Captura)

Requires **Definições → Assistente** enabled.

- [ ] Sidebar **Captura** opens standalone capture screen (search, show processes, create profile from selection).
- [ ] **Captura** tab in editor still works for contextual draft import.
- [ ] When assistant disabled, **Captura** nav is disabled with hint.
- [ ] **Refresh** / list loads candidates without error.
- [ ] Default view shows **apps only** — not `node`/`npm run …`, `obexd`, or LibreOffice `oosplash`.
- [ ] **Show processes** reveals background programs.
- [ ] **Add selected to draft** (editor tab) appends launch rows; **Guardar** persists.
- [ ] Sidebar widget shows **sessão activa** after successful activation; **Desactivar** clears label.

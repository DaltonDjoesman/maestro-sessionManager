# Maestro smoke test checklist (manual)

Use this after `npm run tauri dev` or a release build on **Pop!_OS** / Ubuntu-class Linux with [Tauri prerequisites](https://tauri.app/start/prerequisites/) installed.

Scope note: activation shows an in-app results overlay after hub/editor activate; dry-run/preview is available from hub overflow and the editor (**Pré-visualizar**). Packaging notes: [packaging.md](./packaging.md).

## 1. App starts

- [ ] `npm install` then `npm run tauri dev` opens the Maestro window without Rust panic in the terminal.
- [ ] A **sidebar** shows **Sessões**, **Captura**, **Definições** (and **Editor** while editing); **Sessões** is the default view (session hub).
- [ ] Window **header** shows Maestro logo/title and **theme toggle** (persists via Definições).

## 1b. Visual QA (design handoff)

Compare against `design/maestro-desktop-prototype.html` (inner app window only, pt-PT copy). Portfolio screenshot capture notes: [docs/screenshots/README.md](./screenshots/README.md).

- [ ] **1440×900** — hub cards, sidebar, header match prototype layout.
- [ ] **1366×768**, **1024×768** — no horizontal scroll; drawer sidebar below 900px width.
- [ ] **390×844**, **360×800** — mobile drawer navigation usable.
- [ ] Optional: refresh `docs/screenshots/*.png` from a live build (`node scripts/capture-portfolio-screenshots.mjs` with Vite on :1420, or native window shots per [docs/screenshots/README.md](./screenshots/README.md)).

## 2. Settings

- [ ] **Definições** opens from the sidebar.
- [ ] Settings are grouped (Geral, Sessões, Avançado); **Sobre** appears at the bottom with version (no separate About tab). No global Browser defaults section.
- [ ] Profiles root path validates (existing directory, writable).
- [ ] Theme (system/light/dark) updates the app chrome without restart.
- [ ] Save persists after restart (optional quick check).

## 3. Session hub

- [ ] Hub lists profiles as **grid cards** (prototype `profile-card` layout).
- [ ] Toolbar: search, **Importar** (file picker + **import modal** for display name), **Nova sessão**.
- [ ] Empty catalog shows guidance + **Nova sessão** / **Criar a partir do exemplo** (toolbar Importar/Nova sessão still available).
- [ ] **Ativar** on a valid card runs activation and opens the results overlay (steps + dismiss; **Abrir log** when a log path exists).
- [ ] Overflow **Pré-visualizar** opens preview overlay (argv/labels) **without** spawning processes.
- [ ] Overlay dismiss (Fechar / Esc) returns to a usable hub; failed activate still shows overlay/error feedback.
- [ ] Clicking card body opens the profile editor.
- [ ] Overflow menu (**···**): preview, export (confirmation modal), duplicate, delete.
- [ ] Search filters by name; pins appear in **Fixadas**; last session appears under **Continuar** when valid.

## 4. Profile editor and activation (mock browser)

- [ ] Copy [`docs/examples/smoke-session.profile.json`](docs/examples/smoke-session.profile.json) into your profiles directory (or use hub **Criar a partir do exemplo**).
- [ ] Editor has sticky header (**Guardar**, **Ativar**, **Pré-visualizar**) and tabs **Conteúdo** / **Captura**.
- [ ] **Guardar** works without validation error.
- [ ] **Ativar sessão** invokes activation (apps/browser launch as configured) and opens the same results overlay as the hub.
- [ ] **Pré-visualizar activação** shows planned steps only (no spawns).
- [ ] Invalid profile / bad executable shows failed steps (or invoke error) in the overlay and does not claim silent success.

## 5. Rust tests (CI-friendly)

- [ ] `cd src-tauri && cargo test` passes.

## 6. Running-apps assistant (Captura)

Capture is always available (no settings toggle).

- [ ] Sidebar **Captura** opens standalone capture screen (search, create profile from selection).
- [ ] **Captura** tab in editor works for contextual draft import.
- [ ] Refresh icon loads candidates without error.
- [ ] Default view shows **apps only** — not `node`/`npm run …`, `obexd`, or LibreOffice `oosplash`.
- [ ] When no `desktopWorkspace` values are present, a notice explains grouping is unavailable (list still usable).
- [ ] **Adicionar selecionadas** (editor tab) appends launch rows; **Guardar** persists.
- [ ] Sidebar widget shows **última activação** after successful activation; **Limpar indicador** clears the label only (apps keep running).

## 7. Launcher-honest clear (manual)

- [ ] Activate a session → footer shows the session name.
- [ ] Click **Limpar indicador** → footer clears; launched apps/processes remain running.

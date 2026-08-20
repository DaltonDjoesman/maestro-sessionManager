# Screenshots & demo video

Live UI captures of Maestro (React shell at 1440×900, dark theme, pt-PT). Regenerated from the running Vite UI with a mocked Tauri IPC layer so catalog / capture / activation look filled without personal paths.

## Status

| Asset | Status | Notes |
|-------|--------|-------|
| `hub.png` | Current | Sessões hub with profile cards |
| `capture.png` | Current | Assistente de captura (workspaces) |
| `activation-overlay.png` | Current | Terminal results after **Ativar** |
| `settings.png` | Current | Definições (Geral / Sessões / Avançado) |
| `maestro-activate-demo.mp4` | Current | README demo loop (H.264) |
| `maestro-activate-demo.webm` | Current | README demo loop (VP9 fallback) |

PNG shots = **app window** only. Demo video = activation flow with a visible cursor, then a privacy-safe **full-screen desktop stage** (terminal + Firefox / example.com) showing what the session launched.

## Regenerate screenshots

With `npm run tauri dev` (or at least `npm run dev` on port 1420):

```bash
# one-time if needed
npm install --no-save puppeteer-core
node scripts/capture-portfolio-screenshots.mjs
```

## Regenerate demo video (automated)

Requires Vite on `:1420` and `ffmpeg` on `PATH` (or a binary at `.tools/ffmpeg`):

```bash
npm run dev
npm install --no-save puppeteer-core
node scripts/capture-demo-video.mjs
```

That script:

1. Records hub → **Ativar** on **Demo README** → activation overlay (with a drawn cursor).
2. Appends frames from [`scripts/demo-desktop-stage.html`](../../scripts/demo-desktop-stage.html) (Cosmic-like chrome + terminal + Firefox).
3. Runs [`scripts/optimize-demo-video.sh`](../../scripts/optimize-demo-video.sh) → `maestro-activate-demo.{mp4,webm}`.

Demo profile JSON for a **live** activation: [`docs/examples/demo-recording.profile.json`](../examples/demo-recording.profile.json).

## Capture checklist (native window / OBS)

Prefer a live OBS take when replacing the automated demo with a real desktop recording:

1. Import [`docs/examples/demo-recording.profile.json`](../examples/demo-recording.profile.json) (or copy into your profiles root).
2. Close apps from that profile; dark theme; clean desktop (no personal paths / tabs).
3. Run `npm run tauri dev` on Pop!_OS / Cosmic.
4. OBS: **Screen Capture (PipeWire)** / full monitor, **cursor on**, 30 fps, ~20–30 s.
5. Roteiro: hub pause → click **Ativar** → wait for overlay → **Fechar** → show terminal + browser (~3 s).
6. Optimise:

```bash
START=2 END=28 ./scripts/optimize-demo-video.sh /path/to/raw.mp4
```

Target: combined MP4 + WebM **< 5 MB**.

## Capture checklist (native window stills)

Prefer these when replacing PNG shots with a real Tauri window:

1. Run `npm run tauri dev` on Pop!_OS / Cosmic.
2. Use a clean demo catalog (no personal absolute paths in the shot if possible).
3. Capture hub, captura, activation overlay, and optionally Definições.
4. Crop to the app window; keep theme consistent across shots.

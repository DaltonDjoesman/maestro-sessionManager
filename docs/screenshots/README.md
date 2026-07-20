# Screenshots

Live UI captures of Maestro (React shell at 1440×900, dark theme, pt-PT). Regenerated from the running Vite UI with a mocked Tauri IPC layer so catalog / capture / activation look filled without personal paths.

## Status

| Asset | Status | Notes |
|-------|--------|-------|
| `hub.png` | Current | Sessões hub with profile cards |
| `capture.png` | Current | Assistente de captura (workspaces) |
| `activation-overlay.png` | Current | Terminal results after **Ativar** |
| `settings.png` | Current | Definições (Geral / Sessões / Avançado) |

## Regenerate

With `npm run tauri dev` (or at least `npm run dev` on port 1420):

```bash
# one-time if needed
npm install --no-save puppeteer-core
node scripts/capture-portfolio-screenshots.mjs
```

## Capture checklist (native window)

Prefer these when replacing with a real Tauri window shot:

1. Run `npm run tauri dev` on Pop!_OS / Cosmic.
2. Use a clean demo catalog (no personal absolute paths in the shot if possible).
3. Capture hub, captura, activation overlay, and optionally Definições.
4. Crop to the app window; keep theme consistent across shots.

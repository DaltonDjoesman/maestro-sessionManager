# Screenshots

Live UI captures of Maestro (React shell at 1440×900, dark theme, pt-PT). Regenerated from the running Vite UI with a mocked Tauri IPC layer so catalog / capture / activation look filled without personal paths.

## Status

| Asset | Status | Notes |
|-------|--------|-------|
| `hub.png` | Current | Sessões hub with profile cards |
| `capture.png` | Current | Assistente de captura (workspaces) |
| `activation-overlay.png` | Current | Terminal results after **Ativar** |
| `settings.png` | Current | Definições (Geral / Sessões / Avançado) |
| [`../demo/hub-activate-loop.gif`](../demo/hub-activate-loop.gif) | Current | Short hub → capture → activate loop for README |

## Regenerate

With `npm run tauri dev` (or at least `npm run dev` on port 1420):

```bash
# one-time if needed
npm install --no-save puppeteer-core
node scripts/capture-portfolio-screenshots.mjs
```

That script writes the PNGs under this folder and rebuilds `docs/demo/hub-activate-loop.gif` via ImageMagick (`convert`) from hub → capture → activation frames (720×450, low color count).

### Demo GIF only (from existing PNGs)

If the PNGs are already current and you only need to refresh the motion demo:

```bash
mkdir -p docs/demo
convert \
  \( docs/screenshots/hub.png -resize 720x450 \) \
  \( docs/screenshots/capture.png -resize 720x450 \) \
  \( docs/screenshots/activation-overlay.png -resize 720x450 \) \
  -delay 120 -loop 0 -layers OptimizeTransparency \
  -colors 64 -dither FloydSteinberg \
  docs/demo/hub-activate-loop.gif
```

Requires ImageMagick 6+ (`convert`). Keep the GIF short and quantized so it stays small in git (~30KB).

## Capture checklist (native window)

Prefer these when replacing with a real Tauri window shot:

1. Run `npm run tauri dev` on Pop!_OS / Cosmic.
2. Use a clean demo catalog (no personal absolute paths in the shot if possible).
3. Capture hub, captura, activation overlay, and optionally Definições.
4. Crop to the app window; keep theme consistent across shots.
5. Re-run the ImageMagick command above (or the full capture script) so the README GIF matches.

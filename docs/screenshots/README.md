# Screenshots

Portfolio visuals for the README. Prefer real captures from a running build on Pop!_OS / Cosmic when possible.

## Status

| Asset | Status | Notes |
|-------|--------|-------|
| `hub.png` | Present (interim) | From design handoff; replace with a live app capture when convenient |
| `capture.png` | Pending | Standalone Captura assistant |
| `activation-overlay.png` | Pending | Activation / preview results overlay |

Place PNG (or GIF) files in this folder using the names above, then the README links will resolve.

> Capture and activation overlay shots wait on convenient live captures. Hub is covered by an interim design asset so the README image path resolves.

## Capture checklist

1. Run `npm run tauri dev` (or a release build) on a desktop that matches the target look.
2. Use a clean demo catalog (a few named profiles; avoid personal paths in the shot).
3. Prefer light or dark theme consistently across shots; 1440×900 or similar is fine.
4. Capture:
   - [ ] **Hub** — Sessões hub with at least one profile card visible → save as `hub.png`
   - [ ] **Capture** — Captura list with candidates → `capture.png`
   - [ ] **Activation overlay** — After **Ativar**, results timeline visible → `activation-overlay.png`
5. Crop to the app window (no unrelated desktop clutter).
6. Commit the PNGs and tick the rows in the Status table above.

Prototype reference (layout only): `design/maestro-desktop-prototype.html`.

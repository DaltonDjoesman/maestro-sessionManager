# Follow-up craft (not in this release)

Items from the portfolio roadmap that are intentionally deferred or stubbed:

| Item | Status |
|------|--------|
| CLI `maestro activate <profile>` | **Follow-up change** — activation remains in-app via Tauri commands; no separate CLI binary yet. |
| Global hotkey to activate last/pinned session | **Research stub** — Tauri global shortcut plugins exist; needs UX (which profile?) and Linux compositor permission notes before shipping. |
| Profile templates (“web dev”, “design”, …) | **Follow-up** — empty hub already offers **Criar a partir do exemplo** (`docs/examples/smoke-session.profile.json`). Richer templates can layer on that import path. |

Explicit non-goals remain: open-tab URL scraping, restoring windows onto prior workspaces, session teardown / process kill from the sidebar.

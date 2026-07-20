## Why

Maestro will be published as a **public GitHub portfolio project** (personal demo), not a commercial SaaS. Feature work must stay coherent with a **session launcher** (not window manager / session teardown), improve trust and multi-distro reach, and exclude fragile experiments. This change captures that feature roadmap so implementation can proceed in phases without mixing documentation polish.

## What Changes

- Formalize product positioning in requirements: **launcher-only** — no session teardown; sidebar must not imply “deactivate/kill session.”
- Require **dry-run / preview UI** (backend `preview_session_activation` already exists).
- Add **first-run / empty-hub** guidance with a sample or documented example profile path.
- Polish **import/export** UX (dedicated modals instead of `window.prompt` where practical).
- Expand **multi-distro / multi-desktop** support: packaging docs + tested deb/AppImage intent; Wayland adapters beyond Cosmic (GNOME, then KWin/Hyprland best-effort) with graceful degradation; keep X11/`wmctrl` path documented.
- Optional later craft items: CLI activate, global hotkey, profile templates — listed as phased, not blocking publish.
- **Out of scope (explicit):** auto-fill browser URLs from open tabs; restoring apps onto prior workspaces (X11/Wayland placement); commercial telemetry/auto-update/monetization; reintroducing context-cleanup.
- **Prerequisites (separate changes, not this backlog’s core):** `activation-results-overlay`, CSS/dead-code hygiene if still pending — treat as done or in parallel before multi-distro depth.

## Capabilities

### New Capabilities

- `multi-distro-support`: Packaging and desktop-environment coverage across major Linux families; compositor adapters with honest degradation; maintainer docs for deps per distro family.

### Modified Capabilities

- `application-shell`: Align current-session widget with launcher-only semantics (no false “deactivate session” teardown).
- `activation-preview`: Require a user-facing dry-run/preview control (UI no longer out of scope).
- `session-hub`: First-run / empty state guidance for new users and portfolio demos.
- `profile-portability`: Prefer dedicated import/export UI flows over raw prompts (validation unchanged).

## Impact

- Frontend: shell widget copy/behavior; hub empty state; dry-run entry points; import/export modals; optional CLI is separate binary/surface later.
- Backend/platform: Wayland discovery adapters; packaging config (`tauri.conf.json` targets); no schema break required for launcher-only clarification.
- Specs listed above; docs-only public-repo polish lives in sibling change `portfolio-docs-polish`.
- Large phased apply — expect multiple implementation passes or follow-up changes sliced from these tasks.

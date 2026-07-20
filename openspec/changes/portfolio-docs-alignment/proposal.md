## Why

The portfolio README and docs map are mostly honest, but visitors can over-read the i18n setup as multi-language support, `docs/follow-up-craft.md` is easy to miss, and static screenshots alone make the hub→capture→activate loop harder to grasp than a short demo GIF. Aligning docs with what the app actually delivers improves first impressions on GitHub.

## What Changes

- Clarify in README (and docs map if needed) that **EN is an i18n scaffold only** — default UI is pt-PT; no locale switcher / no shipped English UI yet.
- Link `docs/follow-up-craft.md` from the README docs map (or equivalent).
- Add a short **demo GIF** (or animated WebP) to the README showing hub → capture/activate loop; document how to regenerate beside screenshots.
- Note that duplicate may still use prompt/confirm where modals were not required; keep non-goals table accurate after logging verbosity removal (sibling code change).
- **Out of scope:** translating `en.ts`, implementing CLI/hotkeys, code/CSS hygiene (sibling `portfolio-code-hygiene`).

## Capabilities

### New Capabilities

<!-- None -->

### Modified Capabilities

- `public-project-docs`: README SHALL include a short motion demo (GIF/WebP) of the core loop and SHALL clarify EN scaffold vs default pt-PT UI; follow-up craft doc SHALL be discoverable from the README docs map.
- `ui-localization`: Document that the `en` locale table MAY be a stub alias and SHALL NOT be required to be user-selectable until real English strings exist.

## Impact

- `README.md`, `docs/screenshots/` (or `docs/demo/`), optional script note for GIF capture, docs map links.
- No runtime feature changes required for this change alone (locale copy clarification is documentation + optional README only).

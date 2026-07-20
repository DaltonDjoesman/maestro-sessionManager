## Why

A post-cleanup audit found residual credibility issues for a public portfolio repo: a settings control (`logging_verbosity`) that does nothing, CSS design-system rules shadowed by `App.css`, duplicated basename helpers, aspirational dead adapter stubs, and a frontend with zero automated tests while the Rust core is heavily tested. Fixing these improves honesty and maintainability without adding product surface.

## What Changes

- **Remove** unused `logging_verbosity` / `LogVerbosity` from settings model, UI (Avançado section as needed), types, and persistence (ignore legacy key on load; omit on save). Activation transcript log remains unchanged.
- Resolve CSS conflicts where `.badge`, `.field-error`, and `.form-success` in `App.css` shadow `components.css`; collapse redundant theme rules and obvious orphan selectors where safe.
- Consolidate Rust `executable_basename` / `basename_of` into one shared helper; keep intentional TS↔Rust editor/browser mirrors documented.
- Delete unused compositor stub helpers (`gnome/kwin/hyprland_workspace_enrichment_available`) unless wired for real use in this change (prefer delete).
- Address cheap clippy nits; trim unused test-only / speculative re-exports where clear.
- Add a shared fixture or test asserting Rust and TS profile-normalize stay aligned.
- Introduce minimal Vitest + React Testing Library coverage for pure helpers and critical hub/import paths.
- Lighten `SessionHubPage` / `ProfileEditorScreen` by extracting at least one cohesive hook or subcomponent each (not a full rewrite).
- **Out of scope:** CLI activate, profile templates, GIF/README (sibling `portfolio-docs-alignment`), Wayland adapters, teardown, tab scraping.

## Capabilities

### New Capabilities

<!-- None — hygiene and test infrastructure only. -->

### Modified Capabilities

- `application-settings`: Drop logging verbosity from persisted settings and Settings UI; keep theme, profiles root, and About. Document legacy-key ignore on load.

## Impact

- Backend: `settings/model.rs`, defaults, store tests, Settings UI, `src/types/settings.ts`, possibly empty Avançado section removal.
- CSS: `App.css`, `components.css`, `themes.css`, `tokens.css` as needed.
- Rust capture/activation/browser basename helpers; `platform/adapters.rs` stubs.
- Frontend: Vitest config in `package.json`, new tests under `src/`; modest Hub/Editor extractions.
- Specs: `application-settings` delta only for this change.

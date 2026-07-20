## Why

The project will be a **public GitHub portfolio repo**. Today it lacks a LICENSE, has a developer-oriented README without screenshots or clear positioning, and has no i18n framework beyond a single `pt.ts` file. Documentation and light polish must land so visitors (and recruiters) can understand, build, and trust the project without treating it as a commercial product.

## What Changes

- Add an explicit **LICENSE** and a short note on third-party deps (including GPL-facing Cosmic protocol bindings when distributing binaries).
- Rewrite **README** as a portfolio front door: problem, launcher positioning, stack, screenshots/GIF, quick start, scope/non-goals, links to OpenSpec/docs.
- Add **screenshots** (or placeholders + capture instructions) for hub, capture, and activation overlay.
- Fix package metadata (`Cargo.toml` authors, description consistency).
- Add **CI badge**, minimal **CONTRIBUTING** / personal-project expectations, short **Architecture** section, and a brief **security** note (local JSON profiles, no cloud).
- Introduce an **i18n structure** that can host EN + pt-PT (UI may keep pt-PT as default; README primarily English for portfolio reach).
- Update smoke-test checklist references after overlay/docs land.
- **Out of scope:** implementing multi-distro adapters, dry-run UI, or other product features (see `portfolio-features-roadmap`); F16/F17 experimental features.

## Capabilities

### New Capabilities

- `public-project-docs`: Normative expectations for LICENSE, portfolio README, screenshots, contributing note, architecture overview, and security blurb for a public personal repository.

### Modified Capabilities

- `ui-localization`: Allow a small i18n framework (locale modules + default locale) so additional languages can be added; keep pt-PT as the default UI locale unless settings later choose otherwise.

## Impact

- Repo root: `LICENSE`, `README.md`, `CONTRIBUTING.md`, `docs/` screenshots or `docs/screenshots/`.
- `src/i18n/`: structure for multiple locale files; wire default locale.
- `src-tauri/Cargo.toml` authors/metadata; optional README badge pointing at `.github/workflows/ci.yml`.
- No activation/capture behavior changes required for this change alone.

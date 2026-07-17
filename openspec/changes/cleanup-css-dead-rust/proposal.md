## Why

Two UI generations left orphaned CSS (~100+ unused selectors) and conflicting rules in the stylesheet, while the Rust core still carries dead helpers, a no-op settings flag, and duplicated editor/browser heuristics that can drift across the TS/Rust boundary. Cleaning this now reduces maintenance cost and prevents subtle classification/launch bugs before the next UX work (activation overlay, import modals).

## What Changes

- Remove orphaned legacy CSS (old catalog, session-card grid, running-app cards, duplicate `.hub-toolbar`, unused legacy token aliases) while keeping live design-system primitives actually used by current components.
- Delete or fold dead Rust: unused Cosmic enrichment helpers, legacy process-candidate path superseded by window discovery, unreachable activation delay branch, unused `anyhow` dependency, and `#[allow(dead_code)]` items that are not on an imminent wiring path.
- Remove the no-op `assisted_profile_capture_enabled` settings field (already forced `true` and barred from Settings UI by existing requirements).
- Consolidate duplicated “known editor” basename lists into one Rust source of truth (and one TS mirror or IPC-backed check).
- Consolidate browser-family inference so `hint.rs` and frontend helpers do not disagree with `browser/detect.rs`.
- Remove unused frontend leftovers: orphan i18n keys for retired import-modal/assistant-toggle copy, unused `react.svg`, unused `CreateProfileResult` type.
- **Not in scope:** activation results overlay, dry-run UI, wiring `iconName`/`classificationConfidence` into the capture list, import/export modals, Wayland adapters for other compositors, or reintroducing session cleanup/teardown.

## Capabilities

### New Capabilities

<!-- None — hygiene and consolidation only; no new product surface. -->

### Modified Capabilities

- `application-settings`: Drop the persisted `assisted_profile_capture_enabled` field; capture remains always-on with no Assistente toggle (already required). Document migration/normalization so older settings files still load.

## Impact

- Frontend: `src/App.css`, `src/styles/components.css`, `src/styles/tokens.css`, `src/styles/themes.css`, `src/i18n/pt.ts`, small type/asset cleanup under `src/`.
- Backend: `src-tauri/src/platform/wayland_windows.rs`, `platform/` process helpers, `settings/` model + normalize, `activation/runner.rs`, `process_launcher/`, `browser/hint.rs` + `detect.rs`, `capture/` editor constants, `Cargo.toml` (`anyhow`).
- Specs: delta only for `application-settings`; classifier/icon API fields stay as-is (UI still MAY omit them).
- Risk: CSS deletion must be gated by className audit so live hub/editor/capture styles are not removed; heuristic consolidation must preserve current basename membership for known editors and browser family markers.

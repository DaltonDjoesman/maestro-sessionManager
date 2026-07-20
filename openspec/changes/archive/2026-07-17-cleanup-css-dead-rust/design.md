## Context

Maestro’s current UI (hub cards, window-item capture list, shared `.btn`/`.field` primitives) coexists with large leftover stylesheets from earlier catalog/table and running-app-card UIs. The Rust core marks several unused helpers with `#[allow(dead_code)]`, and `cargo check` still warns about Cosmic enrichment functions that duplicate the live inline path. Editor basenames and browser-family markers are copied across Rust and TypeScript with one inconsistent frontend variant. Settings still persist `assisted_profile_capture_enabled` even though normalize forces it `true` and the Settings UI forbids an Assistente toggle.

## Goals / Non-Goals

**Goals:**

- Shrink CSS to selectors referenced by current `src/**/*.tsx` (plus intentional shared primitives still required by `design-system-handoff`).
- Remove dead Rust paths and the no-op settings field without changing activation/capture user-visible behavior.
- Single canonical editor-basename list and single browser-family detection path to stop cross-language drift.
- Keep `cargo test` / frontend build green; no intentional product UX changes.

**Non-Goals:**

- Activation results overlay, dry-run UI, import/export modals.
- Displaying `iconName` / `classificationConfidence` in the capture list (API fields remain).
- Refactoring activation to fully adopt `process_launcher` sequencing helpers (may delete unreachable delay branch only; larger fold is optional follow-up).
- Session teardown / context-cleanup.
- New Wayland compositor adapters.

## Decisions

| Decision | Choice | Alternatives considered |
|----------|--------|-------------------------|
| CSS deletion rule | Delete a selector only if no `className` / `classList` reference exists in `src/`; keep prototype primitives (`.btn`, `.card`, `.modal*`, `.tab-btn`, etc.) that match `design-system-handoff` even if a modal is not currently mounted | Delete all unused including modal primitives (risk: next overlay/import work re-copies CSS) |
| Duplicate `.hub-toolbar` | Keep the definition that matches the live hub layout; delete the other | Leave both (current silent override is a bug) |
| Settings flag | Remove `assisted_profile_capture_enabled` from the Rust/TS settings model; on load, ignore unknown/legacy keys or strip the field on save | Keep field forever-forced-true (continues schema noise) |
| Editor lists | One `EDITOR_BASENAMES` (or equivalent) in Rust shared by activation skip, window discovery, and assistant; TS uses the same set via a small shared module (or mirrors the exact list once with a comment tying to Rust) | Expose via new Tauri command (heavier for this hygiene change) |
| Browser family | `hint.rs` and any TS `inferBrowserFamily` call into / mirror `detect_browser_family` markers only; no “everything else is Chromium” shortcut that disagrees with detect | Leave crude hint heuristic |
| Cosmic dead helpers | Delete unused apply/match/enrich structs/functions and their tests if only covering the dead path | Call them from the live snapshot path (no product need; inline path already works) |
| `process_launcher` dead helpers | Delete or keep only if activation will call them in this change; prefer delete unreachable runner delay branch without a large activation rewrite | Full activation rewrite onto `spawn_ordered_*` (out of scope unless trivial) |
| Classifier API fields | Keep computing/serializing `iconName` and `classificationConfidence` | Strip from API (**BREAKING** vs specs) — rejected |

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Accidental CSS regression (hub/editor look wrong) | Diff classNames vs selectors before delete; smoke hub, editor, capture, settings after CSS purge |
| Editor/browser consolidation changes who counts as “editor” or “Chromium” | Diff membership of old lists; keep union of intentional markers; run existing classifier/browser tests |
| Older settings JSON still contains the flag | Deserialize with ignore-unknown or drop field in `normalize`; no UI toggle ever |
| Deleting `#[allow(dead_code)]` items someone planned to wire | Proposal already excludes imminent UX; comments that say “reserved for delayed multi-app launch” → remove with unreachable branch, not leave half-dead |

## Migration Plan

1. Settings: load old files → normalize strips/ignores `assisted_profile_capture_enabled` → save without the field.
2. CSS/Rust: pure delete/refactor; no data migration.
3. Rollback: revert the change commit(s); settings files without the field remain valid under “always-on capture”.

## Open Questions

- None blocking: TS mirror of editor basenames vs IPC command can stay as the lighter mirror unless apply discovers painful drift.

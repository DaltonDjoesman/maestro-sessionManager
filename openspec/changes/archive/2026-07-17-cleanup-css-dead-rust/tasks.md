## 1. Settings: drop assisted-capture flag

- [x] 1.1 Remove `assisted_profile_capture_enabled` from Rust settings model and TypeScript `settings` types; update `normalize`/load to ignore legacy key and save without it
- [x] 1.2 Add/adjust Rust tests for loading a legacy settings JSON that still contains the flag and asserting save omits it
- [x] 1.3 Remove unused Assistente i18n keys (`pt.settings.assistant*`) if still present and unreferenced

## 2. CSS hygiene

- [x] 2.1 Audit `className` / class strings in `src/**/*.{tsx,ts}` against selectors in `App.css` and `components.css`; produce delete list of orphans (legacy catalog, session-card, running-app-card, etc.)
- [x] 2.2 Delete orphaned selectors; resolve duplicate `.hub-toolbar` to the live hub definition only
- [x] 2.3 Remove unused legacy CSS variable aliases in `tokens.css` / special-cases in `themes.css` that only served deleted selectors; keep design-system primitives (`.btn`, `.card`, `.modal*`, tabs) required by handoff even if unused today
- [x] 2.4 Visual smoke: hub, editor, capture, settings still layout correctly (no obvious missing styles)

## 3. Dead Rust and dependencies

- [x] 3.1 Delete unused Cosmic helpers flagged by `cargo check` (`apply_cosmic_workspace_to_records`, `match_record_by_title_app_id`, `try_attach_cosmic_workspace_metadata`, `CosmicWorkspaceEnrichment`) and tests that only cover them
- [x] 3.2 Remove superseded `ProcessCandidate` / `list_user_process_candidates_linux` path (and related `#[allow(dead_code)]`) if nothing production calls it
- [x] 3.3 Remove unreachable `delay_after_success = Duration::ZERO` branch in `activation/runner.rs`; delete unused `process_launcher` delay/summary helpers that remain uncalled (or fold only if trivial)
- [x] 3.4 Fold or delete unused `ProfileDirectory::duplicate_file` in favor of `duplicate_file_with_display_name`
- [x] 3.5 Remove unused `anyhow` from `Cargo.toml`; clear other unused re-exports/`#[allow(dead_code)]` items called out in design that are not needed
- [x] 3.6 Ensure `cargo check` is clean of the known dead-code warnings introduced by this cleanup; `cargo test` passes

## 4. Heuristic consolidation

- [x] 4.1 Introduce a single Rust `EDITOR_BASENAMES` (or shared helper) used by activation skip, window discovery, and assistant
- [x] 4.2 Align TypeScript editor checks (`capture.ts`, `RunningAppsCaptureList`) to the same membership rules (exact list mirror + comment, no divergent substring heuristics)
- [x] 4.3 Make `browser/hint.rs` (and TS `browserDetect` if kept) consistent with `detect_browser_family` markers; add/adjust unit tests for agreement on known executables
- [x] 4.4 Optionally merge frontend `normalizeProfile` / `normalizeProfileForRun` into one shared helper if still duplicated

## 5. Frontend leftovers

- [x] 5.1 Delete unused `src/assets/react.svg` and unused `CreateProfileResult` type (or wire the type at call sites if preferred)
- [x] 5.2 Remove remaining unused i18n keys for retired import-modal copy (`pt.import.*` except keys still referenced)

## 6. Verification

- [x] 6.1 Run `cargo test` in `src-tauri`
- [x] 6.2 Run frontend typecheck/build (`npm run build` or project equivalent)
- [x] 6.3 Confirm no Assistente toggle in settings and legacy settings fixture behavior matches the application-settings delta

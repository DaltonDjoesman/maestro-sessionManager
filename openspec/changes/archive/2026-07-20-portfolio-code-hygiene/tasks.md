## 1. Remove inert logging verbosity

- [x] 1.1 Remove `LogVerbosity` / `logging_verbosity` from Rust settings model, defaults, and TypeScript types
- [x] 1.2 Ignore legacy `logging_verbosity` on load; omit on save; update store/model tests
- [x] 1.3 Remove logging control from Settings UI; drop empty **Avançado** section if it has no other fields
- [x] 1.4 Remove related i18n keys if unused

## 2. CSS conflict cleanup

- [x] 2.1 Resolve `.badge` / `.field-error` / `.form-success` shadowing between `App.css` and `components.css` (one source of truth)
- [x] 2.2 Collapse redundant identical light/dark rules in `themes.css` where safe
- [x] 2.3 Remove clearly orphaned selectors found in audit if still unused; smoke hub/editor/settings visuals

## 3. Rust hygiene

- [x] 3.1 Consolidate `executable_basename` / `basename_of` into one shared helper; update call sites
- [x] 3.2 Delete unused `gnome/kwin/hyprland_workspace_enrichment_available` stubs; adjust tests
- [x] 3.3 Fix cheap clippy nits; remove unused test-only helpers/re-exports where clear
- [x] 3.4 `cargo test` and `cargo clippy` clean (or documented remaining allows)

## 4. Normalize parity + Vitest

- [x] 4.1 Add shared golden fixture for legacy-browser-block normalize; assert in Rust and TS
- [x] 4.2 Add Vitest (+ RTL as needed) to the frontend toolchain
- [x] 4.3 Tests for `profileNormalize` and at least one catalog/import helper path
- [x] 4.4 Wire `npm test` (or equivalent) into CI if feasible without breaking Tauri CI

## 5. Hub / Editor light extraction

- [x] 5.1 Extract at least one hook or subcomponent from `SessionHubPage` (e.g. import/export flow)
- [x] 5.2 Extract at least one hook or subcomponent from `ProfileEditorScreen`
- [x] 5.3 Confirm behavior unchanged via smoke or tests

## 6. Verification

- [x] 6.1 Frontend typecheck/build + Vitest
- [x] 6.2 Manual smoke: settings has no logging control; badges/errors still styled; activate still works

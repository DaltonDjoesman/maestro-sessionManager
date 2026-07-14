## 1. Activation preview (backend + parity)

- [x] 1.1 Extract shared “build activation steps” logic used by both execute and preview so `argv` per step is single-sourced.
- [x] 1.2 Add Tauri command `preview_session_activation` (or agreed name) returning ordered steps with `argv` and optional `cwd` without spawning processes.
- [x] 1.3 Add unit or integration coverage asserting preview argv matches execute argv for a representative profile fixture.

## 2. Activation preview (frontend)

- [x] 2.1 Add dry-run entry point on the activation confirmation surface and call the preview command.
- [x] 2.2 Render preview panel: step list with monospace argv, scroll/wrap, loading and error states.

## 3. Activation results UI

- [x] 3.1 Replace or augment post-activation table with vertical timeline and per-step status icons (success, failure, skipped).
- [x] 3.2 Surface human-readable labels from `ActivationStepSummary.kind` (map known kinds; safe fallback string).
- [x] 3.3 Add “Abrir log” using opener plugin when log path exists; toast or inline message when missing.
- [x] 3.4 Ensure activation flow exposes stable log file path to the summary component (prop or store).

## 4. Session catalog UX

- [x] 4.1 Implement `localStorage` persistence for pins and last-opened profile id keyed by short hash of `profiles_root`.
- [x] 4.2 Add “Continuar última sessão” primary action when last id resolves to an existing profile.
- [x] 4.3 Add pin/unpin controls and sort pinned profiles before others with stable pin order.
- [x] 4.4 Add name search filter (case-insensitive substring) on the catalog list.
- [x] 4.5 Add badges: “Browser only” when `browser_only`, and app count when N > 0.

## 5. Theme (application chrome)

- [x] 5.1 Wire `ApplicationSettings.theme` to root attribute (`data-theme`) and `prefers-color-scheme` when theme is `system`.
- [x] 5.2 Audit global CSS tokens for light and dark surfaces; fix low-contrast combinations called out in manual smoke.
- [x] 5.3 Ensure theme changes apply live without full reload (subscribe in app shell / layout).

## 6. Profile portability

- [x] 6.1 Add export action writing current profile JSON including `schema_version` via save dialog or default path per platform conventions.
- [x] 6.2 Add import flow: file picker → validate `schema_version` and required fields → prompt name → write under `profiles_root` with collision handling.
- [x] 6.3 Add “Duplicar” with name prompt and safe filename derivation; refresh catalog after success.
- [x] 6.4 Reject unsupported schema versions with a clear user-facing error (no partial files).

## 7. Linux desktop entry (phase 2)

- [x] 7.1 Add maintainer doc under repo (e.g. `docs/linux-desktop.md`) with `.desktop` template, `Exec`, `StartupWMClass`, and optional MIME association notes.
- [x] 7.2 Align documented `StartupWMClass` / app id with `tauri.conf` identifier used in release builds.

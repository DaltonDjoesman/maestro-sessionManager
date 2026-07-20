## Context

Maestro is a public portfolio Linux session launcher. Settings still expose `logging_verbosity` with no logger backend. CSS has shadowed primitives between `App.css` and `components.css`. Basename extraction is triplicated in Rust. Compositor “available” stubs always return false and are unused in production. Frontend lacks automated tests; Hub/Editor pages are large.

## Goals / Non-Goals

**Goals:**

- Honest settings UI (no inert logging control).
- One winning definition for shared CSS primitives in conflict.
- One Rust basename helper; keep TS mirrors labeled.
- Minimal Vitest suite + normalize parity check.
- Modest extraction from Hub/Editor to reduce god-component pressure.
- Clippy/dead-stub cleanup without product behavior change (except removing the setting).

**Non-Goals:**

- Full Hub/Editor rewrite; CLI; GIF/docs (sibling change); wiring real tracing; new Wayland adapters.

## Decisions

| Decision | Choice | Alternatives |
|----------|--------|--------------|
| Logging setting | Remove field + UI; ignore legacy JSON key on load | Wire tracing (rejected by product choice) |
| Avançado section | Remove if it only held logging; keep section only if other advanced controls remain | Empty section |
| CSS conflict | Prefer `components.css` token-based primitives; delete conflicting `App.css` copies (or vice versa if classNames depend on App.css shapes — verify with className audit) | Leave shadowing |
| Basename | Single helper in `platform` (or `paths`) used by detect/skip/window_discovery | Keep three |
| Adapter stubs | Delete unused `*_workspace_enrichment_available` helpers and adjust tests to assert via `workspace_enrichment_expected` / compositor detection only | Keep forever-false stubs |
| Vitest | Add vitest + RTL; test `profileNormalize`, catalog UI helpers, and one import-modal happy path if cheap | Playwright E2E (heavier) |
| Normalize parity | Golden JSON fixture loaded/asserted in Rust test and TS test (same file under `docs/examples` or `src-tauri/tests/fixtures`) | Manual only |
| Component extract | Extract import/export modal state or activation-trigger helpers first | Full page split |

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Removing logging from schema breaks old settings files | Ignore unknown/legacy key; save without it |
| CSS delete regresses badges | Visual smoke hub/editor; prefer deleting the unused loser after computed-style check |
| Vitest in Tauri project needs careful env | Unit-test pure modules first; mock `invoke` for thin UI tests |
| Over-extracting Hub/Editor | Cap at 1–2 extractions per page |

## Migration Plan

1. Settings load strips/ignores `logging_verbosity`; save omits it.
2. CSS/Rust refactors with tests green.
3. Rollback: revert commit(s); old settings files without the field remain valid.

## Open Questions

- None blocking.

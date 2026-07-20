## Context

Preparing Maestro for a **public personal GitHub repository**. Visitors need license clarity, a portfolio-grade README, visual proof, and a path to add languages without rewriting the app. Product feature work is out of scope here (`portfolio-features-roadmap`).

## Goals / Non-Goals

**Goals:**

- LICENSE + dependency license note for public distribution of source (and awareness for binaries).
- English-first README with launcher positioning, screenshots, quick start, non-goals.
- i18n module layout supporting at least `pt` and `en` string tables; default UI locale remains pt-PT initially.
- Minimal community/meta files (CONTRIBUTING, CI badge, architecture, security blurb).

**Non-Goals:**

- Translating the entire UI to English in the first pass (structure + optional EN table stub is enough).
- Feature implementation (overlay, multi-distro, dry-run).
- Marketing landing page outside the repo.

## Decisions

| Decision | Choice | Alternatives |
|----------|--------|--------------|
| License for project source | MIT (simple portfolio default) unless author prefers Apache-2.0; README notes GPL obligations if redistributing binaries linked to `cosmic-protocols` | Dual-license complexity up front |
| README language | English primary; short PT blurb optional | PT-only README |
| Screenshots | `docs/screenshots/` with real captures when available; README uses relative links | External CDN |
| i18n | `src/i18n/index.ts` + `pt.ts` / `en.ts` (en may mirror keys with TODO English); single `t` accessor | Full i18next immediately |
| Authors | Replace `you` with real author name/handle from git or user-provided | Leave placeholder |

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| GPL confusion | Clear README “Source license vs binary deps” subsection |
| Screenshots stale | Capture after overlay lands; note date in docs |
| EN strings incomplete | Allow en stub; default locale pt so UI does not regress |

## Migration Plan

- Additive files only; README replace in place.
- i18n refactor: update imports from `pt` direct to locale accessor; keep pt strings identical.
- Rollback: revert docs/i18n commits.

## Open Questions

- Exact author string for Cargo/package metadata — use git `user.name` / GitHub handle at apply time if not specified.

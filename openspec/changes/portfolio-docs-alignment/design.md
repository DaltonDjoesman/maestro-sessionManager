## Context

Portfolio docs already include LICENSE, screenshots, positioning, and CI. Gaps: EN i18n looks more complete than it is; follow-up craft doc is orphaned from the README map; no motion demo of the core loop.

## Goals / Non-Goals

**Goals:**

- README clarity on locale reality.
- Discoverable follow-up craft doc.
- Short demo GIF/WebP in README plus regenerate note.
- Stay consistent with launcher non-goals and sibling removal of logging verbosity.

**Non-Goals:**

- Full English UI translation; locale switcher; implementing follow-up craft items.

## Decisions

| Decision | Choice | Alternatives |
|----------|--------|--------------|
| EN wording | Explicit “scaffold / not user-facing yet” in README Stack or i18n note | Translate en.ts now (out of scope) |
| Demo media | GIF or animated WebP under `docs/screenshots/` or `docs/demo/`; ≤ ~5–8s preferred; hub→activate (+ optional capture) | Long MP4 hosting |
| Capture method | Prefer existing mock-IPC screenshot script extended, or short manual `wf-recorder`/Peek capture documented | External marketing video |
| follow-up-craft | Add to README docs map table | Delete file (rejected — keep as backlog) |

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Large GIF in git | Keep short/low-color; document regenerate; optional LFS later if needed |
| GIF goes stale | Same regenerate note as screenshots; capture after UI-stable |
| Overclaiming demo | Show only shipped flows (no fake teardown) |

## Migration Plan

- Additive docs/media; README edit in place.
- Rollback: revert docs commit.

## Open Questions

- Exact recorder tooling on the author’s machine — choose at apply time; document the chosen command in `docs/screenshots/README` or adjacent.

## Context

Portfolio docs already include LICENSE, screenshots, positioning, and CI. Gaps: EN i18n looks more complete than it is; follow-up craft doc is orphaned from the README map.

## Goals / Non-Goals

**Goals:**

- README clarity on locale reality.
- Discoverable follow-up craft doc.
- Stay consistent with launcher non-goals and sibling removal of logging verbosity.

**Non-Goals:**

- Full English UI translation; locale switcher; implementing follow-up craft items.
- Demo GIF / motion media in the README.

## Decisions

| Decision | Choice | Alternatives |
|----------|--------|--------------|
| EN wording | Explicit “scaffold / not user-facing yet” in README Stack or i18n note | Translate en.ts now (out of scope) |
| Demo media | None — static screenshots only | GIF/WebP (rejected) |
| follow-up-craft | Add to README docs map table | Delete file (rejected — keep as backlog) |

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Visitors miss the hub→activate loop | Screenshots table already shows hub, capture, and activation |

## Migration Plan

- README edit in place.
- Rollback: revert docs commit.

## Open Questions

<!-- None -->

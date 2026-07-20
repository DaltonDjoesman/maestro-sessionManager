## Context

Public portfolio goal: Maestro as a **Linux session launcher** (JSON profiles → capture → activate), not a WM or teardown tool. Overlay/feedback and code hygiene are handled in sibling/prior changes. This roadmap sequences remaining **product features**: launcher-honest shell, dry-run UI, first-run, import/export modals, and multi-distro/desktop reach. Explicitly excluded: open-tab URL scraping and workspace window placement.

## Goals / Non-Goals

**Goals:**

- Phased feature backlog with clear priority for a public demo.
- Spec deltas so “launcher puro,” dry-run UI, first-run, portability UX, and multi-distro expectations are normative.
- Honest degradation on non-Cosmic Wayland; packaging guidance for Debian-family and AppImage.

**Non-Goals:**

- Implementing docs/LICENSE/README (sibling `portfolio-docs-polish`).
- Browser tab URL import; restore windows to workspaces.
- Session cleanup/teardown; commercial ops (telemetry, paid tiers).
- Full compositor parity in one apply pass.

## Decisions

| Decision | Choice | Alternatives |
|----------|--------|--------------|
| Positioning | Spec + UI: launcher only; sidebar clears “last activated” label, does not kill processes | Reintroduce teardown |
| Phasing | P1 shell/dry-run/first-run/modals → P2 packaging docs + deb/AppImage verification → P3 GNOME adapter → P4 KWin/Hyprland + CLI/templates | Big-bang multi-distro |
| Dry-run UI | Hub overflow and/or editor control calling existing preview command; optional reuse of results overlay shell for preview steps | Separate preview-only page |
| First-run | Empty hub CTA + link/copy of example profile from `docs/examples/` (ship or copy on first launch) | Full onboarding wizard |
| Multi-distro | Docs + bundle targets first; adapters behind same discovery trait; UI message when workspace grouping unavailable | Claim “works everywhere” |
| Exclusions | Document F16/F17 as rejected for public roadmap | Keep as experimental spikes |

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Roadmap change too large to apply at once | Tasks phased; apply may split into follow-up changes while keeping this as source of truth |
| Wayland adapters stall | Ship packaging + degradation copy before deep GNOME/KWin work |
| Import modal vs cleanup CSS | Coordinate modal primitives with hygiene/overlay changes |

## Migration Plan

- No profile schema break required for launcher-only / dry-run / first-run.
- Multi-distro adapters additive; Cosmic path remains default enrichment when available.
- Rollback per phase by reverting that phase’s commits.

## Open Questions

- Whether first-run copies example profile into `profiles_root` automatically or only offers “create from example” — default: offer explicit action to avoid surprising writes.

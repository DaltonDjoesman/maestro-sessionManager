## MODIFIED Requirements

### Requirement: README SHALL serve as a portfolio front door

The README SHALL describe the problem Maestro solves, state that it is a **session launcher** (not a window manager or session teardown tool), list the stack (Tauri 2, React, Rust, OpenSpec), provide quick-start build instructions, link to key docs, and embed or link screenshots of the main surfaces (hub, capture, activation results when available). The README docs map (or equivalent links section) SHALL include `docs/follow-up-craft.md` when that file exists. The README SHALL state that the default UI locale is pt-PT and that any English locale file is a scaffold only until real translations and a user-facing locale switch exist.

#### Scenario: Positioning is explicit

- **WHEN** a visitor reads the README introduction
- **THEN** they SHALL see clear launcher-only positioning and explicit non-goals (no window placement control, no process teardown)

#### Scenario: Screenshots linked

- **WHEN** a visitor reads the README
- **THEN** they SHALL find screenshots or a screenshots folder link for at least the session hub

#### Scenario: Follow-up craft discoverable

- **WHEN** a visitor follows the README documentation links
- **THEN** they SHALL be able to reach `docs/follow-up-craft.md` without browsing the tree blindly

#### Scenario: Locale expectations clear

- **WHEN** a visitor reads the README stack or i18n note
- **THEN** they SHALL understand that pt-PT is the default UI and English is not a shipped selectable locale yet

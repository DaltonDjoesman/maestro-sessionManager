## ADDED Requirements

### Requirement: Public repository SHALL include an explicit LICENSE

The repository root SHALL contain a LICENSE file for the project’s own source. Project documentation SHALL briefly note third-party dependency license implications relevant to redistribution (including GPL-licensed Cosmic protocol bindings when shipping binaries that link them).

#### Scenario: LICENSE present at root

- **WHEN** a visitor opens the repository root
- **THEN** a LICENSE file SHALL be present and identifiable

#### Scenario: Dependency license note visible

- **WHEN** a visitor reads the README or linked docs section on licensing
- **THEN** they SHALL find a short note about third-party/GPL implications for binary distribution

### Requirement: README SHALL serve as a portfolio front door

The README SHALL describe the problem Maestro solves, state that it is a **session launcher** (not a window manager or session teardown tool), list the stack (Tauri 2, React, Rust, OpenSpec), provide quick-start build instructions, link to key docs, and embed or link screenshots of the main surfaces (hub, capture, activation results when available).

#### Scenario: Positioning is explicit

- **WHEN** a visitor reads the README introduction
- **THEN** they SHALL see clear launcher-only positioning and explicit non-goals (no window placement control, no process teardown)

#### Scenario: Screenshots linked

- **WHEN** a visitor reads the README
- **THEN** they SHALL find screenshots or a screenshots folder link for at least the session hub

### Requirement: Repo meta docs SHALL set expectations

The repository SHALL include a short CONTRIBUTING (or equivalent) note marking the project as a personal portfolio effort, a CI status badge when CI exists, a brief architecture overview (app shell → IPC → activation/capture), and a short security note (local profiles, no cloud accounts).

#### Scenario: Personal project expectations

- **WHEN** a contributor opens CONTRIBUTING or the README community section
- **THEN** they SHALL understand the project is personal/portfolio-scoped and how to report issues or propose small fixes

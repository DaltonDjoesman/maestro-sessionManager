## MODIFIED Requirements

### Requirement: Scored app vs process classification

The system SHALL classify each running assistant candidate using a scored model. Classification SHALL NOT depend on a hardcoded allowlist of executable basenames as the primary mechanism. A candidate SHALL be classified as `app` when its score meets or exceeds the configured application threshold after noise exclusion. A candidate SHALL be classified as `process` when below the threshold.

When a window source returned a non-empty mapped window list for the session, a process that matches a `.desktop` entry but has **no** mapped top-level window SHALL be classified as `process` (unless other explicit product rules apply, such as known editor cwd handling).

When **no** window source produced mapped windows (typical pure Wayland without protocol access), a process that matches a `.desktop` entry, passes noise/denylist filters, and would otherwise only fail the threshold because of the no-window penalty SHALL be classified as `app`.

#### Scenario: Window-backed app without hardcoded basename

- **WHEN** a running program has a mapped top-level window and no denylist match, even if its executable basename is not in any static allowlist
- **THEN** the candidate SHALL have `kind` equal to `app`

#### Scenario: Background daemon with desktop file but no window while windows are enumerable

- **WHEN** a window source returned at least one mapped window, and a process matches a `.desktop` entry but has no mapped top-level window and is not a known editor with special cwd handling
- **THEN** the candidate SHALL have `kind` equal to `process`

#### Scenario: Strong desktop match on Wayland without window API

- **WHEN** `XDG_SESSION_TYPE` is `wayland`, no mapped window list is available, and a running process matches an installed `.desktop` entry and passes noise filters
- **THEN** the candidate SHALL have `kind` equal to `app`

#### Scenario: Hardcoded basename allowlist is not required

- **WHEN** the build ships without a static GUI basename allowlist (or with an empty allowlist)
- **THEN** the assistant SHALL still classify deb-installed and Flatpak-installed GUI applications that have windows or strong `.desktop` matches as `app`

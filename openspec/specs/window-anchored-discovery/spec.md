# window-anchored-discovery Specification

## Purpose
TBD - created by archiving change universal-app-classification. Update Purpose after archive.
## Requirements
### Requirement: Window-first candidate discovery on X11

On Linux when top-level window enumeration is available (for example `wmctrl -l -p` on X11), the assistant SHALL derive primary candidates from mapped windows before falling back to orphan process enumeration. Each mapped window SHALL produce at least one assistant row with `desktopWorkspace` when the compositor reports a workspace index.

#### Scenario: AppImage with window but weak desktop Exec

- **WHEN** an AppImage process owns a mapped window and has no matching `.desktop` Exec entry
- **THEN** the assistant SHALL still list the program as an `app` candidate with `displayName` derived from the window title or executable basename

#### Scenario: Multiple windows same PID on different workspaces

- **WHEN** one browser process owns windows on workspace indices 1 and 4
- **THEN** the assistant SHALL emit separate candidates (or clearly distinct rows) per window/workspace combination rather than collapsing to a single row

### Requirement: Window title metadata

When discovery is window-anchored, the system SHALL populate optional `windowTitle` on the candidate for UI disambiguation.

#### Scenario: Vivaldi windows disambiguated by title

- **WHEN** two Vivaldi windows have different titles on different workspaces
- **THEN** each candidate SHALL include `windowTitle` reflecting the respective window title

### Requirement: Process fallback when windows unavailable

When window enumeration is unavailable (for example pure Wayland without compositor API), the system SHALL fall back to process enumeration with scored classification and expanded `.desktop` indexing without failing the assistant command.

#### Scenario: Wayland session without window API

- **WHEN** `XDG_SESSION_TYPE` is `wayland` and no supported window list is available
- **THEN** the assistant SHALL return candidates from process scan and SHALL omit `windowTitle` and workspace grouping when not resolvable


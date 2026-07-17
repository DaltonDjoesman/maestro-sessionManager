# window-anchored-discovery Specification

## Purpose
TBD - created by archiving change universal-app-classification. Update Purpose after archive.

## Requirements

### Requirement: Window-first candidate discovery on X11

On Linux when top-level window enumeration is available — including `wmctrl -l -p` on X11 **and** a supported Wayland foreign-toplevel (or equivalent) window source — the assistant SHALL derive primary candidates from mapped windows before falling back to orphan process enumeration. Each mapped window SHALL produce at least one assistant row with `desktopWorkspace` when the compositor or window source reports a workspace index.

#### Scenario: AppImage with window but weak desktop Exec

- **WHEN** an AppImage process owns a mapped window and has no matching `.desktop` Exec entry
- **THEN** the assistant SHALL still list the program as an `app` candidate with `displayName` derived from the window title or executable basename

#### Scenario: Multiple windows same PID on different workspaces

- **WHEN** one browser process owns windows on workspace indices 1 and 4
- **THEN** the assistant SHALL emit separate candidates (or clearly distinct rows) per window/workspace combination rather than collapsing to a single row

#### Scenario: Wayland session with foreign toplevel windows

- **WHEN** `XDG_SESSION_TYPE` is `wayland` and a supported Wayland window source returns mapped toplevels
- **THEN** the assistant SHALL use those windows for window-first discovery the same way it uses `wmctrl` records on X11

### Requirement: Window title metadata

When discovery is window-anchored, the system SHALL populate optional `windowTitle` on the candidate for UI disambiguation.

#### Scenario: Vivaldi windows disambiguated by title

- **WHEN** two Vivaldi windows have different titles on different workspaces
- **THEN** each candidate SHALL include `windowTitle` reflecting the respective window title

### Requirement: Process fallback when windows unavailable

When window enumeration is unavailable (for example pure Wayland without compositor API and without usable XWayland/`wmctrl` records), the system SHALL fall back to process enumeration with scored classification and expanded `.desktop` indexing without failing the assistant command. On that fallback path, candidates that strongly match an installed `.desktop` entry and pass noise filters SHALL be classifiable as `app` (see `app-classifier-scoring`).

#### Scenario: Wayland session without window API

- **WHEN** `XDG_SESSION_TYPE` is `wayland` and no supported window list is available
- **THEN** the assistant SHALL return candidates from process scan, SHALL omit `windowTitle` and workspace grouping when not resolvable, and SHALL still surface strong `.desktop` matches as `app` when they meet the application score threshold after noise exclusion

### Requirement: Supplemental XWayland window records on Wayland

On Wayland sessions, when `wmctrl -l -p` (or equivalent EWMH listing) succeeds for XWayland clients, the system MAY merge those window records into the window index as a supplemental source. Supplemental XWayland records SHALL NOT be treated as complete coverage of all open apps.

#### Scenario: Slack on XWayland while Cursor is native Wayland

- **WHEN** the session is Wayland, `wmctrl` lists an XWayland Slack window, and native Wayland apps are also running
- **THEN** the assistant SHALL include the Slack window in window-anchored discovery when present, and SHALL still discover native Wayland apps via foreign-toplevel and/or process + `.desktop` fallback

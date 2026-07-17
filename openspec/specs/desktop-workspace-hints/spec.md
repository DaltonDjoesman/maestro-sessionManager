# desktop-workspace-hints Specification

## Purpose
TBD - created by archiving change running-apps-assistant-quality. Update Purpose after archive.

## Requirements

### Requirement: Best-effort desktop workspace read (Linux)

On Linux, when the active session exposes workspace metadata for top-level windows, the system MAY attach an optional `desktopWorkspace` identifier to assistant candidates. Workspace SHALL be resolved per **window** when window-anchored discovery is active, not inferred solely from a deduplicated process row with a different PID than the window owner. Sources MAY include EWMH `_NET_WM_DESKTOP` via `wmctrl` on X11/XWayland **and** Cosmic (or other) Wayland toplevel workspace metadata when available. The system SHALL NOT move, focus, or resize windows. When workspace cannot be resolved, `desktopWorkspace` SHALL be omitted or null.

#### Scenario: X11 session resolves workspace index

- **WHEN** `XDG_SESSION_TYPE` is `x11` (or `x11` fallback) and EWMH `_NET_WM_DESKTOP` is available for a window owned by the candidate PID
- **THEN** the candidate SHALL include `desktopWorkspace` as a non-negative integer workspace index

#### Scenario: Wayland session without compositor API

- **WHEN** `XDG_SESSION_TYPE` is `wayland` and no supported compositor workspace API is available
- **THEN** candidates SHALL be returned without `desktopWorkspace` and the assistant SHALL remain fully functional

#### Scenario: Cosmic Wayland session with toplevel workspace metadata

- **WHEN** `XDG_SESSION_TYPE` is `wayland`, Cosmic (or another supported adapter) reports a workspace index for a mapped toplevel used in discovery
- **THEN** the corresponding candidate SHALL include `desktopWorkspace` as that non-negative integer index

#### Scenario: Multi-window browser rows carry distinct workspaces

- **WHEN** window-anchored discovery emits two rows for the same executable on different workspaces
- **THEN** each row SHALL carry the `desktopWorkspace` of its respective window

### Requirement: Assistant UI groups by desktop workspace

When one or more candidates include `desktopWorkspace`, the profile editor assistant SHALL render candidates grouped under workspace headings (for example “Workspace 1”, “Workspace 2”) ordered by workspace index. Candidates without workspace SHALL appear under a single “No workspace” or equivalent group after numbered workspaces.

#### Scenario: Mixed workspace and unknown candidates

- **WHEN** the user refreshes the assistant and some candidates have `desktopWorkspace` and others do not
- **THEN** the UI SHALL show numbered workspace sections first in ascending order and SHALL show candidates without workspace in a separate trailing section

#### Scenario: No workspace metadata available

- **WHEN** no candidate includes `desktopWorkspace`
- **THEN** the UI SHALL fall back to the phase-1 layout (flat or alphabetical grouping by `displayName`) without error

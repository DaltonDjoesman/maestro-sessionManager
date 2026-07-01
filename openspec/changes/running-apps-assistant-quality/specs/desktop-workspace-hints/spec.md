## ADDED Requirements

### Requirement: Best-effort desktop workspace read (Linux)

On Linux, when the active session exposes workspace metadata for top-level windows, the system MAY attach an optional `desktopWorkspace` identifier to assistant candidates that own a resolvable window. The system SHALL NOT move, focus, or resize windows. When workspace cannot be resolved, `desktopWorkspace` SHALL be omitted or null.

#### Scenario: X11 session resolves workspace index

- **WHEN** `XDG_SESSION_TYPE` is `x11` (or `x11` fallback) and EWMH `_NET_WM_DESKTOP` is available for a window owned by the candidate PID
- **THEN** the candidate SHALL include `desktopWorkspace` as a non-negative integer workspace index

#### Scenario: Wayland session without compositor API

- **WHEN** `XDG_SESSION_TYPE` is `wayland` and no supported compositor workspace API is available
- **THEN** candidates SHALL be returned without `desktopWorkspace` and the assistant SHALL remain fully functional

### Requirement: Assistant UI groups by desktop workspace

When one or more candidates include `desktopWorkspace`, the profile editor assistant SHALL render candidates grouped under workspace headings (for example “Workspace 1”, “Workspace 2”) ordered by workspace index. Candidates without workspace SHALL appear under a single “No workspace” or equivalent group after numbered workspaces.

#### Scenario: Mixed workspace and unknown candidates

- **WHEN** the user refreshes the assistant and some candidates have `desktopWorkspace` and others do not
- **THEN** the UI SHALL show numbered workspace sections first in ascending order and SHALL show candidates without workspace in a separate trailing section

#### Scenario: No workspace metadata available

- **WHEN** no candidate includes `desktopWorkspace`
- **THEN** the UI SHALL fall back to the phase-1 layout (flat or alphabetical grouping by `displayName`) without error

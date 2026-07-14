## MODIFIED Requirements

### Requirement: Best-effort desktop workspace read (Linux)

On Linux, when the active session exposes workspace metadata for top-level windows, the system MAY attach an optional `desktopWorkspace` identifier to assistant candidates. Workspace SHALL be resolved per **window** when window-anchored discovery is active, not inferred solely from a deduplicated process row with a different PID than the window owner. The system SHALL NOT move, focus, or resize windows. When workspace cannot be resolved, `desktopWorkspace` SHALL be omitted or null.

#### Scenario: X11 session resolves workspace index

- **WHEN** `XDG_SESSION_TYPE` is `x11` (or `x11` fallback) and EWMH `_NET_WM_DESKTOP` is available for a window owned by the candidate PID
- **THEN** the candidate SHALL include `desktopWorkspace` as a non-negative integer workspace index

#### Scenario: Wayland session without compositor API

- **WHEN** `XDG_SESSION_TYPE` is `wayland` and no supported compositor workspace API is available
- **THEN** candidates SHALL be returned without `desktopWorkspace` and the assistant SHALL remain fully functional

#### Scenario: Multi-window browser rows carry distinct workspaces

- **WHEN** window-anchored discovery emits two rows for the same executable on different workspaces
- **THEN** each row SHALL carry the `desktopWorkspace` of its respective window

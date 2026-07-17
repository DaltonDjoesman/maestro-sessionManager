## MODIFIED Requirements

### Requirement: Best-effort desktop workspace read (Linux)

On Linux, when the active session exposes workspace metadata for top-level windows, the system MAY attach an optional `desktopWorkspace` identifier to assistant candidates. Workspace SHALL be resolved per **window** when window-anchored discovery is active, not inferred solely from a deduplicated process row with a different PID than the window owner. Sources MAY include EWMH `_NET_WM_DESKTOP` via `wmctrl` on X11/XWayland **and** Cosmic Wayland toplevel/workspace protocols (`zcosmic_toplevel_info_v1` plus workspace-handle→index mapping) when available. The system SHALL NOT move, focus, or resize windows. When workspace cannot be resolved, `desktopWorkspace` SHALL be omitted or null — the system SHALL NOT invent a workspace index of `0` solely because a Wayland title/`app_id` was found.

#### Scenario: X11 session resolves workspace index

- **WHEN** `XDG_SESSION_TYPE` is `x11` (or `x11` fallback) and EWMH `_NET_WM_DESKTOP` is available for a window owned by the candidate PID
- **THEN** the candidate SHALL include `desktopWorkspace` as a non-negative integer workspace index

#### Scenario: Wayland session without compositor API

- **WHEN** `XDG_SESSION_TYPE` is `wayland` and no supported compositor workspace API is available (or Cosmic workspace attachment soft-fails)
- **THEN** candidates SHALL be returned without `desktopWorkspace` and the assistant SHALL remain fully functional

#### Scenario: Cosmic Wayland session with toplevel workspace metadata

- **WHEN** `XDG_SESSION_TYPE` is `wayland`, Cosmic reports workspace membership for a mapped toplevel used in discovery, and that membership maps to a stable 0-based index
- **THEN** the corresponding candidate SHALL include `desktopWorkspace` as that non-negative integer index

#### Scenario: Distinct Cosmic workspaces produce distinct groups

- **WHEN** window-anchored discovery on Cosmic emits two mapped windows whose Cosmic workspace indices differ
- **THEN** the two candidates SHALL carry different `desktopWorkspace` values so the UI can render separate workspace headings

#### Scenario: Multi-window browser rows carry distinct workspaces

- **WHEN** window-anchored discovery emits two rows for the same executable on different workspaces
- **THEN** each row SHALL carry the `desktopWorkspace` of its respective window

## ADDED Requirements

### Requirement: Cosmic workspace soft-fail does not fake Workspace 1

When Cosmic foreign-toplevel enumeration succeeds but Cosmic workspace metadata cannot be attached, candidates from those Wayland records SHALL omit `desktopWorkspace` rather than defaulting every row to index `0` (which the UI would render as a single “Workspace 1” group).

#### Scenario: Titles without workspace indices

- **WHEN** Cosmic foreign-toplevel returns mapped windows with titles/`app_id` but workspace handle mapping fails or is unavailable
- **THEN** those candidates SHALL have `desktopWorkspace` omitted or null and the assistant SHALL still list them as apps when classification succeeds

## MODIFIED Requirements

### Requirement: Wayland top-level window enumeration

On Linux Wayland sessions, the system SHALL attempt to enumerate mapped top-level windows through a compositor-supported foreign-toplevel mechanism (for example `ext-foreign-toplevel-list-v1`, optionally extended by Cosmic `zcosmic_toplevel_info_v1`). Successful enumeration SHALL produce window records usable by window-anchored discovery (title and/or application id). When Cosmic (or another supported adapter) also provides workspace membership for a toplevel, the record SHALL include a stable 0-based workspace index; when workspace membership is unavailable, the record SHALL leave workspace unset. Failure or denial of foreign-toplevel or Cosmic workspace protocols SHALL NOT fail the assistant command.

#### Scenario: Cosmic grants foreign toplevel list

- **WHEN** `XDG_SESSION_TYPE` is `wayland`, the session is Cosmic (or another compositor advertising foreign toplevel), and the protocol returns one or more mapped toplevels
- **THEN** the assistant SHALL treat those toplevels as mapped windows for candidate discovery (including `windowTitle` when a title is available)

#### Scenario: Cosmic attaches workspace indices

- **WHEN** Cosmic foreign-toplevel enumeration succeeds and Cosmic toplevel-info/workspace protocols report workspace membership for those toplevels
- **THEN** each corresponding window record SHALL carry a non-negative 0-based workspace index derived from a stable handle→index map for that snapshot

#### Scenario: Protocol unavailable or denied

- **WHEN** `XDG_SESSION_TYPE` is `wayland` and foreign-toplevel binding fails, times out, or returns no windows
- **THEN** the assistant SHALL continue with process fallback (and any supplemental XWayland/`wmctrl` records) without raising a fatal error

#### Scenario: Cosmic workspace protocol soft-fails

- **WHEN** foreign-toplevel binding succeeds but Cosmic workspace enrichment fails, times out, or is not advertised
- **THEN** the assistant SHALL keep title/`app_id` window records without workspace indices and SHALL NOT fail the assistant command

## ADDED Requirements

### Requirement: Cosmic workspace enrichment stays isolated

Cosmic-specific workspace protocol details SHALL remain behind the Linux Wayland platform adapter (`wayland_windows` / related modules). Shared capture DTOs SHALL only see optional numeric `desktopWorkspace` (or equivalent), not Cosmic handles or protocol objects. Unsupported compositors SHALL no-op Cosmic enrichment.

#### Scenario: Non-Cosmic Wayland compositor

- **WHEN** the session is Wayland but Cosmic toplevel-info/workspace globals are not present
- **THEN** Cosmic workspace enrichment SHALL no-op and foreign-toplevel (if any) SHALL proceed without Cosmic indices

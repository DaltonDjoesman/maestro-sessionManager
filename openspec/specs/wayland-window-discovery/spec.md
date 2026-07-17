# wayland-window-discovery Specification

## Purpose
Discover mapped top-level windows on Linux Wayland sessions via compositor adapters (Cosmic first), with graceful no-op when unsupported.

## Requirements

### Requirement: Wayland top-level window enumeration

On Linux Wayland sessions, the system SHALL attempt to enumerate mapped top-level windows through a compositor-supported foreign-toplevel mechanism (for example `ext-foreign-toplevel-list-v1`, optionally extended by Cosmic `zcosmic_toplevel_info_v1`). Successful enumeration SHALL produce window records usable by window-anchored discovery (title and/or application id, and workspace index when the compositor provides one). Failure or denial of the protocol SHALL NOT fail the assistant command.

#### Scenario: Cosmic grants foreign toplevel list

- **WHEN** `XDG_SESSION_TYPE` is `wayland`, the session is Cosmic (or another compositor advertising foreign toplevel), and the protocol returns one or more mapped toplevels
- **THEN** the assistant SHALL treat those toplevels as mapped windows for candidate discovery (including `windowTitle` when a title is available)

#### Scenario: Protocol unavailable or denied

- **WHEN** `XDG_SESSION_TYPE` is `wayland` and foreign-toplevel binding fails, times out, or returns no windows
- **THEN** the assistant SHALL continue with process fallback (and any supplemental XWayland/`wmctrl` records) without raising a fatal error

### Requirement: Compositor adapter isolation

Wayland window enumeration SHALL be implemented behind a Linux platform adapter so Cosmic-specific protocol details do not leak into shared capture DTOs. Unsupported compositors SHALL no-op and rely on process + `.desktop` classification.

#### Scenario: Unsupported Wayland compositor

- **WHEN** the session is Wayland but no known adapter can list toplevels
- **THEN** window enumeration from the Wayland adapter SHALL yield an empty list and discovery SHALL degrade per `window-anchored-discovery` process fallback rules

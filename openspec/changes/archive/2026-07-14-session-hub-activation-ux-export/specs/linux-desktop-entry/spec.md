## ADDED Requirements

### Requirement: Linux packaging SHALL document a .desktop entry for “Open with Maestro”

Project documentation (and optionally packaging scripts) SHALL describe a `.desktop` file with stable `Exec` pointing to the installed Maestro binary, `MimeType` associations for `application/x-maestro-session` (or agreed extension), and `StartupWMClass` matching the app id used by Tauri.

#### Scenario: Desktop file references bundle id

**WHEN** a maintainer follows the documented template  
**THEN** the resulting `.desktop` SHALL use the same application identifier as the packaged Tauri app so the window manager groups windows correctly.

### Requirement: Phase 2 SHALL register optional file association

After MVP validation, installers MAY register the MIME type and default application for session JSON files so double-click opens Maestro with import preselected; this SHALL remain optional and documented as post-MVP.

#### Scenario: Association is opt-in

**WHEN** the user installs without file association hooks  
**THEN** Maestro SHALL still run normally and SHALL not require MIME registration.

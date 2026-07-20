## ADDED Requirements

### Requirement: Packaging SHALL target common Linux distribution forms

The project SHALL document and aim to produce installable artifacts suitable for Debian-family systems (`.deb`) and a portable AppImage (or equivalent) so users on multiple distributions can run Maestro without building from source. Documentation SHALL list build prerequisites for at least Debian/Ubuntu-family and note Fedora/Arch dependency names at a maintainer level.

#### Scenario: Deb and AppImage documented

- **WHEN** a visitor follows the packaging section of the project docs
- **THEN** they SHALL find instructions to build `.deb` and AppImage (or documented limitations if a target is unavailable on the builder host)

#### Scenario: Distro dependency families listed

- **WHEN** a packager needs native libraries for Tauri on Linux
- **THEN** docs SHALL cover Debian/Ubuntu-family packages and SHALL provide equivalent package-name hints for at least one other family (Fedora or Arch)

### Requirement: Window discovery SHALL degrade honestly outside Cosmic

On Wayland compositors without a Maestro adapter, assisted capture SHALL continue via process and `.desktop` scoring (and foreign-toplevel when available) without failing the assistant command. The UI SHALL indicate when desktop workspace grouping is unavailable rather than inventing a single fake workspace.

#### Scenario: Unsupported compositor keeps assistant usable

- **WHEN** the session is Wayland on an unsupported compositor for workspace enrichment
- **THEN** listing running apps SHALL still succeed and workspace grouping SHALL omit fabricated indices

### Requirement: Additional compositor adapters MAY be added incrementally

The platform layer SHALL allow adding Wayland adapters (for example GNOME, KWin, Hyprland) behind the existing discovery boundary without requiring Cosmic-specific types in shared DTOs. Absence of an adapter SHALL NOT block packaging for that distro.

#### Scenario: Adapter isolation

- **WHEN** a new compositor adapter is introduced
- **THEN** shared capture DTOs SHALL continue to expose only portable fields (such as optional `desktopWorkspace`) and Cosmic-specific protocol types SHALL remain adapter-local

## MODIFIED Requirements

### Requirement: Candidate kind classification

The system SHALL classify each running assistant candidate as `app` or `process` before returning it to the UI. Classification SHALL use a scored model (see `app-classifier-scoring`) combining Freedesktop `.desktop` metadata, mapped top-level windows, and shared noise denylist rules. Processes matching built-in noise or infrastructure denylist rules SHALL NOT appear in the candidate list at all. **The system SHALL NOT use a hardcoded allowlist of GUI executable basenames as the primary classification path.**

#### Scenario: Desktop entry match yields app kind

- **WHEN** a running process executable matches a installed `.desktop` entry `Exec`, `TryExec`, or `StartupWMClass` association and meets the application score threshold
- **THEN** the candidate SHALL have `kind` equal to `app` and SHALL include a human-readable `displayName` from the desktop entry `Name` field when available

#### Scenario: Unmatched user process yields process kind

- **WHEN** a running process passes noise filters but does not meet the application score threshold
- **THEN** the candidate SHALL have `kind` equal to `process` and SHALL use executable basename as `displayName` fallback

#### Scenario: Infrastructure daemon excluded entirely

- **WHEN** a running process matches expanded noise rules (for example Bluetooth `obexd`, LibreOffice splash `oosplash`, or `node` invoked only for `npm run` / `npx` development tooling)
- **THEN** the candidate SHALL NOT be included in the assistant response

### Requirement: Freedesktop display metadata

For candidates classified as `app`, the system SHALL populate optional `iconName` from the matched desktop entry `Icon` field when present. The UI MAY render a themed icon or placeholder from `iconName`; absence of icon resolution SHALL NOT fail the list command.

#### Scenario: Icon name forwarded for matched app

- **WHEN** a candidate is classified as `app` and the matched desktop entry defines `Icon=obsidian`
- **THEN** the candidate SHALL include `iconName` with value `obsidian`

## ADDED Requirements

### Requirement: Expanded desktop entry index

The system SHALL index `.desktop` files from at minimum: `/usr/share/applications`, `~/.local/share/applications`, Flatpak export paths under `/var/lib/flatpak/exports/share/applications` and `~/.local/share/flatpak/exports/share/applications`, and Snap desktop paths under `/var/lib/snapd/desktop/applications` when those directories exist.

#### Scenario: Flatpak application discovered on fresh Pop install

- **WHEN** a Flatpak app is installed and exports a `.desktop` file under Flatpak exports
- **THEN** a running instance of that app SHALL match the desktop entry without manual basename allowlisting

### Requirement: Robust Exec line parsing

When parsing desktop `Exec` lines, the system SHALL extract executable keys usable for matching after common wrappers including Flatpak `bwrap` and similar spawn wrappers.

#### Scenario: Flatpak bwrap Exec matches running process

- **WHEN** a `.desktop` `Exec` line begins with `bwrap` and contains `/app/<id>` as the application binary
- **THEN** the index SHALL register `<id>` as a match key for running processes whose executable path contains that segment

## REMOVED Requirements

### Requirement: Static GUI basename allowlist fallback

**Reason**: Replaced by scored classification using windows and expanded Freedesktop index; hardcoded lists do not scale across machines and install methods.

**Migration**: Remove `is_known_gui_basename` and equivalent constants; rely on `app-classifier-scoring` and `window-anchored-discovery`. Existing users see the same default “apps only” filter with improved coverage.

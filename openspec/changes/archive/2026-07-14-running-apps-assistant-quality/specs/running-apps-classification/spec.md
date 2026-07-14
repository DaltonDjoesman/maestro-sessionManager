## ADDED Requirements

### Requirement: Candidate kind classification

The system SHALL classify each running assistant candidate as `app` or `process` before returning it to the UI. Classification SHALL use Freedesktop `.desktop` metadata when a reasonable match exists between the process executable or argv0 and a installed desktop entry. Processes matching built-in noise or infrastructure denylist rules SHALL NOT appear in the candidate list at all.

#### Scenario: Desktop entry match yields app kind

- **WHEN** a running process executable matches a installed `.desktop` entry `Exec` or basename association
- **THEN** the candidate SHALL have `kind` equal to `app` and SHALL include a human-readable `displayName` from the desktop entry `Name` field when available

#### Scenario: Unmatched user process yields process kind

- **WHEN** a running process passes noise filters but does not match any desktop entry
- **THEN** the candidate SHALL have `kind` equal to `process` and SHALL use executable basename as `displayName` fallback

#### Scenario: Infrastructure daemon excluded entirely

- **WHEN** a running process matches expanded noise rules (for example Bluetooth `obexd`, LibreOffice splash `oosplash`, or `node` invoked only for `npm run` / `npx` development tooling)
- **THEN** the candidate SHALL NOT be included in the assistant response

### Requirement: Expanded process noise filtering

The Linux assistant pipeline SHALL extend shared denylist and background-noise rules to exclude common non-application processes on Pop!_OS / GNOME / Cosmic reference systems without hiding processes the user explicitly enables via the advanced toggle.

#### Scenario: Node development server filtered

- **WHEN** a `node` process command line indicates `npm run` or `npx` without a matched desktop application entry
- **THEN** the candidate SHALL NOT be included in the default assistant response

#### Scenario: Bluetooth obex daemon filtered

- **WHEN** process basename is `obexd`
- **THEN** the candidate SHALL NOT be included in the assistant response

### Requirement: Freedesktop display metadata

For candidates classified as `app`, the system SHALL populate optional `iconName` from the matched desktop entry `Icon` field when present. The UI MAY render a themed icon or placeholder from `iconName`; absence of icon resolution SHALL NOT fail the list command.

#### Scenario: Icon name forwarded for matched app

- **WHEN** a candidate is classified as `app` and the matched desktop entry defines `Icon=obsidian`
- **THEN** the candidate SHALL include `iconName` with value `obsidian`

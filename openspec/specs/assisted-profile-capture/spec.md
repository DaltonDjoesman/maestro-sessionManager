# assisted-profile-capture Specification

## Purpose
TBD - created by archiving change maestro-mvp. Update Purpose after archive.
## Requirements
### Requirement: Running applications suggestion list

The system SHALL provide an assistant that lists candidate user applications currently running, filtered to reduce system noise. By default the list SHALL include only candidates classified as `app` using the scored universal classifier. The user MAY enable an advanced control to also show candidates classified as `process`. The user SHALL select zero or more candidates to add as launch entries to a draft profile. Each candidate SHALL expose `displayName` for UI labels; when `windowTitle` is present it SHALL be used to disambiguate multiple rows of the same application. When enabled in settings, the assistant SHALL be presented in the profile editor **Captura** tab (not as an inline section above applications in the main scroll).

#### Scenario: User adds a candidate to draft

- **WHEN** the user selects a running candidate and confirms add to draft
- **THEN** the draft profile SHALL gain an application launch entry using the best-known executable and SHALL not persist until the user saves the profile

#### Scenario: Default list hides processes

- **WHEN** the user opens the assistant with default settings
- **THEN** the UI SHALL show only candidates with `kind` equal to `app`

#### Scenario: Advanced toggle shows processes

- **WHEN** the user enables “Show processes” (or equivalent advanced control)
- **THEN** the UI SHALL also list candidates with `kind` equal to `process` in a visually distinct section

#### Scenario: Multiple browser windows listed separately

- **WHEN** window-anchored discovery returns two `app` rows for the same browser executable on different workspaces
- **THEN** the UI SHALL display both rows and SHALL allow independent selection for draft import

#### Scenario: Assistant in Capture tab

- **WHEN** assisted capture is enabled and the user opens the **Captura** tab
- **THEN** the running-apps list and selection UI SHALL be available there

### Requirement: Editor working directory hint (Linux)

On Linux, the system MAY inspect process metadata (for example current working directory of known editor processes) to suggest a project folder for an IDE launch entry. Any suggestion SHALL be labeled as non-authoritative and SHALL remain editable before save. Editor deduplication by workspace folder SHALL continue to apply independently of desktop workspace grouping.

#### Scenario: Suggestion is editable

- **WHEN** the assistant suggests a working directory for a known editor process
- **THEN** the user SHALL be able to change or clear that path before the entry is written to JSON

### Requirement: Post-minimal-MVP delivery

The assisted-profile-capture capability MAY ship after the first installable MVP that satisfies session activation without this assistant. The catalog and manual profile editing SHALL remain sufficient for users until this capability is enabled.

#### Scenario: Assistant disabled in early build

- **WHEN** the build flag or product configuration disables the assistant
- **THEN** the system SHALL hide assistant entry points and SHALL not fail other capabilities

### Requirement: Assistant section labeling and presentation

The profile editor assistant section SHALL use user-facing copy that reflects application selection (for example “Running apps”) rather than generic process enumeration. Cards SHALL prioritize `displayName`, optional icon, and project folder hint over raw PID and full command line.

#### Scenario: Card shows human name first

- **WHEN** a candidate has `displayName` “Obsidian” and executable `/app/obsidian`
- **THEN** the card title SHALL display “Obsidian” and SHALL NOT use only the executable basename as the primary title


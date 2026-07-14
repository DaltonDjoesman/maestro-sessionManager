## MODIFIED Requirements

### Requirement: Running applications suggestion list

The system SHALL provide an assistant that lists candidate user applications currently running, filtered to reduce system noise. By default the list SHALL include only candidates classified as `app` (see `running-apps-classification`). The user MAY enable an advanced control to also show candidates classified as `process`. The user SHALL select zero or more candidates to add as launch entries to a draft profile. Each candidate SHALL expose `displayName` for UI labels; technical fields (`executable`, `cmdPreview`, `pid`) SHALL remain available but SHALL NOT be the primary visual label.

#### Scenario: User adds a candidate to draft

- **WHEN** the user selects a running candidate and confirms add to draft
- **THEN** the draft profile SHALL gain an application launch entry using the best-known executable and SHALL not persist until the user saves the profile

#### Scenario: Default list hides processes

- **WHEN** the user opens the assistant with default settings
- **THEN** the UI SHALL show only candidates with `kind` equal to `app`

#### Scenario: Advanced toggle shows processes

- **WHEN** the user enables “Show processes” (or equivalent advanced control)
- **THEN** the UI SHALL also list candidates with `kind` equal to `process` in a visually distinct section

### Requirement: Editor working directory hint (Linux)

On Linux, the system MAY inspect process metadata (for example current working directory of known editor processes) to suggest a project folder for an IDE launch entry. Any suggestion SHALL be labeled as non-authoritative and SHALL remain editable before save. Editor deduplication by workspace folder SHALL continue to apply independently of desktop workspace grouping.

#### Scenario: Suggestion is editable

- **WHEN** the assistant suggests a working directory for a known editor process
- **THEN** the user SHALL be able to change or clear that path before the entry is written to JSON

## ADDED Requirements

### Requirement: Assistant section labeling and presentation

The profile editor assistant section SHALL use user-facing copy that reflects application selection (for example “Running apps”) rather than generic process enumeration. Cards SHALL prioritize `displayName`, optional icon, and project folder hint over raw PID and full command line.

#### Scenario: Card shows human name first

- **WHEN** a candidate has `displayName` “Obsidian” and executable `/app/obsidian`
- **THEN** the card title SHALL display “Obsidian” and SHALL NOT use only the executable basename as the primary title

## ADDED Requirements

### Requirement: Running applications suggestion list

The system SHALL provide an assistant that lists candidate user applications currently running, filtered to reduce system noise. The user SHALL select zero or more candidates to add as launch entries to a draft profile.

#### Scenario: User adds a candidate to draft

- **WHEN** the user selects a running candidate and confirms add to draft
- **THEN** the draft profile SHALL gain an application launch entry using the best-known executable and SHALL not persist until the user saves the profile

### Requirement: Editor working directory hint (Linux)

On Linux, the system MAY inspect process metadata (for example current working directory of known editor processes) to suggest a project folder for an IDE launch entry. Any suggestion SHALL be labeled as non-authoritative and SHALL remain editable before save.

#### Scenario: Suggestion is editable

- **WHEN** the assistant suggests a working directory for a known editor process
- **THEN** the user SHALL be able to change or clear that path before the entry is written to JSON

### Requirement: Post-minimal-MVP delivery

The assisted-profile-capture capability MAY ship after the first installable MVP that satisfies session activation without this assistant. The catalog and manual profile editing SHALL remain sufficient for users until this capability is enabled.

#### Scenario: Assistant disabled in early build

- **WHEN** the build flag or product configuration disables the assistant
- **THEN** the system SHALL hide assistant entry points and SHALL not fail other capabilities

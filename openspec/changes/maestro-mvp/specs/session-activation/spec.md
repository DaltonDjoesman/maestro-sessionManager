## ADDED Requirements

### Requirement: Activate session flow

The system SHALL provide a single user action to activate a named session profile. Activation SHALL validate the profile, resolve paths, execute the browser launch block if present, then execute application launch entries in profile order unless a documented alternate ordering is configured. The system SHALL return a structured summary to the UI listing each step with status success, failure, or warning.

#### Scenario: Full session activation

- **WHEN** the user activates a valid profile that includes both browser URLs and two application entries
- **THEN** the system SHALL perform browser launch (if enabled) and both application launches and SHALL return a summary containing three step results

#### Scenario: Validation failure before spawn

- **WHEN** the profile fails validation (for example missing executable on a required entry)
- **THEN** the system SHALL not spawn partial steps unless explicitly configured otherwise and SHALL return validation errors

### Requirement: Optional skip-if-already-running

The system MAY support a per-entry or global policy to skip launching an application when a process with the same executable basename is already running. If implemented, the skipped step SHALL appear in the summary with status skipped and a reason.

#### Scenario: Skip policy hits running process

- **WHEN** skip-if-running is enabled for an entry and a matching process exists
- **THEN** the system SHALL not spawn a duplicate process for that entry and SHALL record skipped in the summary

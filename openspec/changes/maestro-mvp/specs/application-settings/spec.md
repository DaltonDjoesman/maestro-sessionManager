## ADDED Requirements

### Requirement: Global settings persistence

The system SHALL persist global application settings to a local file (JSON or Tauri plugin store) with a `schema_version`. Settings SHALL include at minimum: profiles root directory path; default browser executable or detection hint; optional default browser family; logging verbosity; UI theme preference.

#### Scenario: Load settings on startup

- **WHEN** the application starts and a valid settings file exists
- **THEN** the system SHALL load settings before showing the main catalog and SHALL use the configured profiles directory

#### Scenario: First run creates defaults

- **WHEN** no settings file exists on first launch
- **THEN** the system SHALL create default settings pointing to a sensible per-user data directory and SHALL persist them on first successful profile access or explicit save from settings UI

### Requirement: Settings UI

The system SHALL provide a settings screen to edit global settings. Invalid paths SHALL be rejected with inline validation errors before save.

#### Scenario: Invalid profiles path

- **WHEN** the user enters a profiles path that is not a directory or is not writable
- **THEN** the system SHALL refuse to save and SHALL display the validation error

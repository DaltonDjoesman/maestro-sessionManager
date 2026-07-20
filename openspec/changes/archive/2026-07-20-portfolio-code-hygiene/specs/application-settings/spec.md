## MODIFIED Requirements

### Requirement: Global settings persistence

The system SHALL persist global application settings to a local file (JSON or Tauri plugin store) with a `schema_version`. Settings SHALL include at minimum: profiles root directory path and UI theme preference. Settings SHALL NOT include a `logging_verbosity` (or equivalent) field. When loading a settings file that still contains a legacy logging-verbosity key, the system SHALL ignore that field and SHALL persist settings without it on the next successful save.

#### Scenario: Load settings on startup

- **WHEN** the application starts and a valid settings file exists
- **THEN** the system SHALL load settings before showing the main catalog and SHALL use the configured profiles directory

#### Scenario: First run creates defaults

- **WHEN** no settings file exists on first launch
- **THEN** the system SHALL create default settings pointing to a sensible per-user data directory and SHALL persist them on first successful profile access or explicit save from settings UI

#### Scenario: Legacy logging verbosity key ignored

- **WHEN** a settings file from an older build contains `logging_verbosity`
- **THEN** the application SHALL load successfully and SHALL omit that field on the next successful save

### Requirement: Settings UI

The system SHALL provide a settings screen grouped into sections styled per the prototype: **Geral** (theme) and **Sessões** (profiles directory). An **Avançado** section SHALL NOT be required solely for logging verbosity. Invalid paths SHALL be rejected with inline validation errors before save. An **About** block with application version SHALL appear at the bottom of the settings screen; there SHALL NOT be a separate About route.

Running-apps capture is always enabled; there SHALL NOT be a separate **Assistente** toggle in settings; settings persistence SHALL NOT retain an assisted-capture enablement flag.

There SHALL NOT be global default browser executable or family fields in settings; browser configuration lives on session profile application entries (and system browser detection may inform editor placeholders only).

There SHALL NOT be a logging-verbosity control in Definições.

#### Scenario: Invalid profiles path

- **WHEN** the user enters a profiles path that is not a directory or is not writable
- **THEN** the system SHALL refuse to save and SHALL display the validation error

#### Scenario: About in settings footer

- **WHEN** the user navigates to Definições
- **THEN** version and short product description SHALL be visible without a separate About navigation item

#### Scenario: No logging verbosity control

- **WHEN** the user opens Definições
- **THEN** no logging-verbosity dropdown or equivalent control SHALL be shown

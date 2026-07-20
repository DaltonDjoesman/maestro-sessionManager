## ADDED Requirements

### Requirement: Settings schema SHALL omit assisted capture toggle field

The persisted application settings document SHALL NOT include an `assisted_profile_capture_enabled` (or equivalent) field. Running-apps capture remains always enabled. When loading a settings file that still contains a legacy assisted-capture flag, the system SHALL ignore that field and SHALL persist settings without it on the next successful save.

#### Scenario: Legacy settings file with assisted flag loads

- **WHEN** a settings file from an older build contains `assisted_profile_capture_enabled`
- **THEN** the application SHALL load successfully and SHALL NOT expose an Assistente toggle in Definições

#### Scenario: Save drops legacy assisted flag

- **WHEN** settings are saved after loading a file that contained `assisted_profile_capture_enabled`
- **THEN** the written settings document SHALL omit that field

## MODIFIED Requirements

### Requirement: Settings UI

The system SHALL provide a settings screen grouped into sections styled per the prototype: **Geral** (theme), **Sessões** (profiles directory), **Browser** (defaults), and **Avançado** (logging verbosity). Each section SHALL use prototype form-section titles and field spacing. Invalid paths SHALL be rejected with inline validation errors before save. An **About** block with application version SHALL appear at the bottom of the settings screen; there SHALL NOT be a separate About route.

Running-apps capture is always enabled; there SHALL NOT be a separate **Assistente** toggle in settings; settings persistence SHALL NOT retain an assisted-capture enablement flag.

#### Scenario: Invalid profiles path

- **WHEN** the user enters a profiles path that is not a directory or is not writable
- **THEN** the system SHALL refuse to save and SHALL display the validation error

#### Scenario: About in settings footer

- **WHEN** the user navigates to Definições
- **THEN** version and short product description SHALL be visible without a separate About navigation item

#### Scenario: No assisted capture toggle or persisted flag

- **WHEN** the user opens Definições
- **THEN** no Assistente enablement control SHALL be shown and saving settings SHALL not write an assisted-capture enablement field

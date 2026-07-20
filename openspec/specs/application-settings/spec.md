# application-settings Specification

## Purpose

Global Maestro settings persistence and settings UI.

## Requirements

### Requirement: Global settings persistence

The system SHALL persist global application settings to a local file (JSON or Tauri plugin store) with a `schema_version`. Settings SHALL include at minimum: profiles root directory path; logging verbosity; UI theme preference.

#### Scenario: Load settings on startup

- **WHEN** the application starts and a valid settings file exists
- **THEN** the system SHALL load settings before showing the main catalog and SHALL use the configured profiles directory

#### Scenario: First run creates defaults

- **WHEN** no settings file exists on first launch
- **THEN** the system SHALL create default settings pointing to a sensible per-user data directory and SHALL persist them on first successful profile access or explicit save from settings UI

### Requirement: Settings UI

The system SHALL provide a settings screen grouped into sections styled per the prototype: **Geral** (theme), **Sessões** (profiles directory), and **Avançado** (logging verbosity). Each section SHALL use prototype form-section titles and field spacing. Invalid paths SHALL be rejected with inline validation errors before save. An **About** block with application version SHALL appear at the bottom of the settings screen; there SHALL NOT be a separate About route.

Running-apps capture is always enabled; there SHALL NOT be a separate **Assistente** toggle in settings; settings persistence SHALL NOT retain an assisted-capture enablement flag.

There SHALL NOT be global default browser executable or family fields in settings; browser configuration lives on session profile application entries (and system browser detection may inform editor placeholders only).

#### Scenario: Invalid profiles path

- **WHEN** the user enters a profiles path that is not a directory or is not writable
- **THEN** the system SHALL refuse to save and SHALL display the validation error

#### Scenario: About in settings footer

- **WHEN** the user navigates to Definições
- **THEN** version and short product description SHALL be visible without a separate About navigation item

#### Scenario: No assisted capture toggle or persisted flag

- **WHEN** the user opens Definições
- **THEN** no Assistente enablement control SHALL be shown and saving settings SHALL not write an assisted-capture enablement field

#### Scenario: No global browser defaults section

- **WHEN** the user opens Definições
- **THEN** no Browser section for default executable or family SHALL be shown

### Requirement: Settings schema SHALL omit assisted capture toggle field

The persisted application settings document SHALL NOT include an `assisted_profile_capture_enabled` (or equivalent) field. Running-apps capture remains always enabled. When loading a settings file that still contains a legacy assisted-capture flag, the system SHALL ignore that field and SHALL persist settings without it on the next successful save.

#### Scenario: Legacy settings file with assisted flag loads

- **WHEN** a settings file from an older build contains `assisted_profile_capture_enabled`
- **THEN** the application SHALL load successfully and SHALL NOT expose an Assistente toggle in Definições

#### Scenario: Save drops legacy assisted flag

- **WHEN** settings are saved after loading a file that contained `assisted_profile_capture_enabled`
- **THEN** the written settings document SHALL omit that field

### Requirement: Settings schema SHALL omit global browser defaults

The persisted application settings document SHALL NOT include `default_browser_executable` or `default_browser_family`. When loading a settings file that still contains those legacy fields, the system SHALL ignore them and SHALL persist settings without them on the next successful save.

#### Scenario: Legacy browser defaults load and drop on save

- **WHEN** a settings file from an older build contains `default_browser_executable` and/or `default_browser_family`
- **THEN** the application SHALL load successfully, SHALL NOT show those fields in Definições, and SHALL omit them from the document on the next successful save

### Requirement: Theme selection SHALL affect application chrome

When `ApplicationSettings.theme` is `light` or `dark`, the webview root SHALL set `data-theme` and apply OKLCH prototype tokens to the window header, sidebar, cards, and forms. When `system`, the app SHALL follow OS preference.

#### Scenario: Light theme readable contrast

- **WHEN** theme is `light`
- **THEN** body text and interactive elements SHALL meet at least the same contrast baseline as the dark theme for primary surfaces (no unreadable grey-on-white defaults)

#### Scenario: System theme tracks OS

- **WHEN** theme is `system` and the OS switches light/dark
- **THEN** the application chrome SHALL update without requiring an app restart

#### Scenario: Dark theme applies tokens

- **WHEN** the user selects dark theme in Definições and saves
- **THEN** the document root SHALL set `data-theme="dark"` and sidebar, cards, and forms SHALL use dark design tokens

#### Scenario: Settings theme matches header toggle

- **WHEN** the user changes theme in Definições
- **THEN** the header theme toggle state and all surfaces SHALL reflect the saved theme immediately

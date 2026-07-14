# application-settings Specification

## Purpose
TBD - created by archiving change maestro-mvp. Update Purpose after archive.
## Requirements
### Requirement: Global settings persistence

The system SHALL persist global application settings to a local file (JSON or Tauri plugin store) with a `schema_version`. Settings SHALL include at minimum: profiles root directory path; default browser executable or detection hint; optional default browser family; logging verbosity; UI theme preference.

#### Scenario: Load settings on startup

- **WHEN** the application starts and a valid settings file exists
- **THEN** the system SHALL load settings before showing the main catalog and SHALL use the configured profiles directory

#### Scenario: First run creates defaults

- **WHEN** no settings file exists on first launch
- **THEN** the system SHALL create default settings pointing to a sensible per-user data directory and SHALL persist them on first successful profile access or explicit save from settings UI

### Requirement: Settings UI

The system SHALL provide a settings screen grouped into sections: **Geral** (theme), **Sessões** (profiles directory), **Browser** (defaults), **Assistente** (running-apps toggle), and **Avançado** (logging verbosity). Invalid paths SHALL be rejected with inline validation errors before save. An **About** block with application version SHALL appear at the bottom of the settings screen; there SHALL NOT be a separate About route.

#### Scenario: Invalid profiles path

- **WHEN** the user enters a profiles path that is not a directory or is not writable
- **THEN** the system SHALL refuse to save and SHALL display the validation error

#### Scenario: About in settings footer

- **WHEN** the user navigates to Definições
- **THEN** version and short product description SHALL be visible without a separate About navigation item

### Requirement: Theme selection SHALL affect application chrome

When `ApplicationSettings.theme` is `light` or `dark`, the webview root SHALL set `data-theme` and apply design tokens to the sidebar, cards, and forms. When `system`, the app SHALL follow OS preference.

#### Scenario: Light theme readable contrast

- **WHEN** theme is `light`
- **THEN** body text and interactive elements SHALL meet at least the same contrast baseline as the dark theme for primary surfaces (no unreadable grey-on-white defaults)

#### Scenario: System theme tracks OS

- **WHEN** theme is `system` and the OS switches light/dark
- **THEN** the application chrome SHALL update without requiring an app restart

#### Scenario: Dark theme applies tokens

- **WHEN** the user selects dark theme in Definições and saves
- **THEN** the document root SHALL set `data-theme="dark"` and sidebar, cards, and forms SHALL use dark design tokens


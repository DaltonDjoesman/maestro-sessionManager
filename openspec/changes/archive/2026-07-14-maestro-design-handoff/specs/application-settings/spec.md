## MODIFIED Requirements

### Requirement: Settings UI

The system SHALL provide a settings screen grouped into sections styled per the prototype: **Geral** (theme), **Sessões** (profiles directory), **Browser** (defaults), **Assistente** (running-apps toggle), and **Avançado** (logging verbosity). Each section SHALL use prototype form-section titles and field spacing. Invalid paths SHALL be rejected with inline validation errors before save. An **About** block with application version SHALL appear at the bottom of the settings screen; there SHALL NOT be a separate About route.

#### Scenario: Invalid profiles path

- **WHEN** the user enters a profiles path that is not a directory or is not writable
- **THEN** the system SHALL refuse to save and SHALL display the validation error

#### Scenario: About in settings footer

- **WHEN** the user navigates to Definições
- **THEN** version and short product description SHALL be visible without a separate About navigation item

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

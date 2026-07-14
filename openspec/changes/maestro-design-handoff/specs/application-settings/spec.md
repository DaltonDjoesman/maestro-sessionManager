## MODIFIED Requirements

### Requirement: Settings UI

The system SHALL provide a settings screen grouped into sections styled per the prototype: **Geral** (theme), **Sessões** (profiles directory), **Browser** (defaults), **Assistente** (running-apps toggle), and **Avançado** (logging verbosity). Each section SHALL use prototype form-section titles and field spacing. Invalid paths SHALL be rejected with inline validation errors before save. An **About** block with application version SHALL appear at the bottom of the settings screen; there SHALL NOT be a separate About route.

#### Scenario: About in settings footer

- **WHEN** the user navigates to Definições
- **THEN** version and short product description SHALL be visible without a separate About navigation item

### Requirement: Theme selection SHALL affect application chrome

When `ApplicationSettings.theme` is `light` or `dark`, the webview root SHALL set `data-theme` and apply OKLCH prototype tokens to the window header, sidebar, cards, and forms. When `system`, the app SHALL follow OS preference.

#### Scenario: Settings theme matches header toggle

- **WHEN** the user changes theme in Definições
- **THEN** the header theme toggle state and all surfaces SHALL reflect the saved theme immediately

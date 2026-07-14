## ADDED Requirements

### Requirement: Theme selection SHALL affect application chrome

When `ApplicationSettings.theme` is `light` or `dark`, the webview root SHALL set `data-theme` (or equivalent) and apply design tokens so backgrounds, text, borders, and primary controls match the selected theme. When `system`, the app SHALL follow OS preference until the user changes the setting.

#### Scenario: Light theme readable contrast

**WHEN** theme is `light`  
**THEN** body text and interactive elements SHALL meet at least the same contrast baseline as the dark theme for primary surfaces (no unreadable grey-on-white defaults).

#### Scenario: System theme tracks OS

**WHEN** theme is `system` and the OS switches light/dark  
**THEN** the application chrome SHALL update without requiring an app restart.

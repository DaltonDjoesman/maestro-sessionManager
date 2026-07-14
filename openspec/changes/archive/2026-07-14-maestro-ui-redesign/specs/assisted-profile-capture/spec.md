## MODIFIED Requirements

### Requirement: Running applications suggestion list

The system SHALL provide an assistant that lists candidate user applications currently running, filtered to reduce system noise. By default the list SHALL include only candidates classified as `app` using the scored universal classifier. The user MAY enable an advanced control to also show candidates classified as `process`. The user SHALL select zero or more candidates to add as launch entries to a draft profile. Each candidate SHALL expose `displayName` for UI labels; when `windowTitle` is present it SHALL be used to disambiguate multiple rows of the same application. When enabled in settings, the assistant SHALL be presented in the profile editor **Captura** tab (not as an inline section above applications in the main scroll).

#### Scenario: User adds a candidate to draft

- **WHEN** the user selects a running candidate and confirms add to draft
- **THEN** the draft profile SHALL gain an application launch entry using the best-known executable and SHALL not persist until the user saves the profile

#### Scenario: Default list hides processes

- **WHEN** the user opens the assistant with default settings
- **THEN** the UI SHALL show only candidates with `kind` equal to `app`

#### Scenario: Advanced toggle shows processes

- **WHEN** the user enables “Show processes” (or equivalent advanced control)
- **THEN** the UI SHALL also list candidates with `kind` equal to `process` in a visually distinct section

#### Scenario: Multiple browser windows listed separately

- **WHEN** window-anchored discovery returns two `app` rows for the same browser executable on different workspaces
- **THEN** the UI SHALL display both rows and SHALL allow independent selection for draft import

#### Scenario: Assistant in Capture tab

- **WHEN** assisted capture is enabled and the user opens the **Captura** tab
- **THEN** the running-apps list and selection UI SHALL be available there

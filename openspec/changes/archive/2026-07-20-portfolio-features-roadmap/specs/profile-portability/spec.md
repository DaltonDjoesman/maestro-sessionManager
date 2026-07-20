## MODIFIED Requirements

### Requirement: Users SHALL export a session profile to JSON

From the profile editor or catalog context menu, the user SHALL be able to export the current profile to a JSON file including `schema_version` and canonical fields. Export SHALL present a dedicated confirmation UI (modal or save dialog flow) showing what will be written rather than an unexplained silent write alone.

#### Scenario: Export writes valid JSON

- **WHEN** export is chosen for a profile
- **THEN** the written file SHALL parse as JSON and SHALL include `schema_version` equal to the profile’s version

#### Scenario: Export uses dedicated UI

- **WHEN** the user chooses export from the hub overflow or editor
- **THEN** the app SHALL present a dedicated export UI or system save dialog before or as part of writing the file

### Requirement: Users SHALL import a session profile from JSON

The app SHALL allow picking a JSON file, validating `schema_version` and required fields, then saving as a new profile file under the active `profiles_root` with a user-chosen name (subject to filename safety rules). Import SHALL use a dedicated import UI for the display name and errors (not solely `window.prompt`).

#### Scenario: Import rejects unknown schema

- **WHEN** imported JSON has an unsupported `schema_version`
- **THEN** the app SHALL refuse import with a clear error and SHALL not write a partial profile file

#### Scenario: Import succeeds

- **WHEN** imported JSON matches a supported schema and the chosen name is unique under `profiles_root`
- **THEN** a new profile file SHALL appear and the catalog SHALL list it without restart

#### Scenario: Import UI collects display name

- **WHEN** the user imports a valid JSON profile
- **THEN** the dedicated import UI SHALL collect the display name before saving

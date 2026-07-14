## MODIFIED Requirements

### Requirement: Post-activation summary SHALL present a readable timeline

After activation completes, the UI SHALL render steps as a vertical timeline with status icons (success, failure, skipped, warning) and human-readable labels. The summary SHALL appear in a **shared panel** (modal or sheet) accessible from both the session hub and the profile editor, not only inline at the bottom of the editor.

#### Scenario: Results from hub activation

- **WHEN** activation is triggered from the session hub
- **THEN** the shared results panel SHALL open with the timeline

#### Scenario: Failed step highlighted

- **WHEN** any step has `status` failure
- **THEN** that step SHALL be visually distinct and the overall outcome SHALL read as failed until dismissed

### Requirement: Post-activation summary SHALL link to the activation log

The shared summary SHALL include **Abrir log** that opens the log file path used for activation output when such a file exists; if it does not exist yet, the app SHALL show a non-blocking message explaining that logging may be disabled or not yet created.

#### Scenario: Open log succeeds

- **WHEN** the log file path is known and the file exists
- **THEN** choosing **Abrir log** SHALL open it with the system default application for that file type

#### Scenario: Open log missing file

- **WHEN** the log path is known but the file does not exist
- **THEN** the app SHALL not crash and SHALL inform the user that the log file is unavailable

#### Scenario: Open log from results panel

- **WHEN** activation completes and the user clicks **Abrir log** in the shared results panel
- **THEN** the system SHALL open the activation log file path when available, or show a non-blocking message when no log path exists

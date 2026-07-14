## ADDED Requirements

### Requirement: Post-activation summary SHALL present a readable timeline

After activation completes, the UI SHALL render steps as a vertical timeline with status icons (success, failure, skipped) and human-readable labels matching `ActivationStepSummary.kind`.

#### Scenario: Failed step highlighted

**WHEN** any step has `status` failure  
**THEN** that step SHALL be visually distinct and the overall outcome SHALL read as failed until dismissed.

### Requirement: Post-activation summary SHALL link to the activation log

The summary SHALL include an action “Abrir log” that opens the log file path used for activation output when such a file exists; if it does not exist yet, the app SHALL show a non-blocking message explaining that logging may be disabled or not yet created.

#### Scenario: Open log succeeds

**WHEN** the log file path is known and the file exists  
**THEN** choosing “Abrir log” SHALL open it with the system default application for that file type.

#### Scenario: Open log missing file

**WHEN** the log path is known but the file does not exist  
**THEN** the app SHALL not crash and SHALL inform the user that the log file is unavailable.

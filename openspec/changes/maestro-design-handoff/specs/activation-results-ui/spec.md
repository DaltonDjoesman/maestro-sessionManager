## MODIFIED Requirements

### Requirement: Post-activation summary SHALL present a readable timeline

After activation completes, the UI SHALL render steps as a vertical timeline with status icons (success, failure, skipped, warning) and human-readable labels inside a **terminal-style overlay** matching the exported prototype (dark log surface, monospace step output, dismiss control). The overlay SHALL be accessible from both the session hub and the profile editor.

#### Scenario: Results from hub activation

- **WHEN** activation is triggered from the session hub
- **THEN** the terminal overlay SHALL open with the timeline and log output

#### Scenario: Failed step highlighted

- **WHEN** any step has `status` failure
- **THEN** that step SHALL be visually distinct in the overlay and the overall outcome SHALL read as failed until dismissed

### Requirement: Post-activation summary SHALL link to the activation log

The terminal overlay SHALL include **Abrir log** that opens the log file path when available; otherwise a non-blocking message within the overlay.

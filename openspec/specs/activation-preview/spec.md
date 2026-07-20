# activation-preview Specification

## Purpose

Backend dry-run / preview of activation steps without executing them, plus a user-facing dry-run control in the hub and/or editor.

## Requirements

### Requirement: The system SHALL expose a preview-only activation command

The backend SHALL provide a command (e.g. `preview_session_activation`) that returns the ordered list of planned activation steps including `argv` (and optional `cwd`) for each step **without** spawning processes or performing irreversible side effects.

#### Scenario: Preview returns argv

- **WHEN** the client invokes preview for a valid profile
- **THEN** the response SHALL include each planned step with `argv` as it would be used by the real activation path

#### Scenario: Preview does not execute

- **WHEN** preview is invoked
- **THEN** no child processes for activation steps SHALL be started and no activation log SHALL be appended for real execution

### Requirement: Dry-run UI SHALL be available from hub or editor

The session hub and/or profile editor SHALL expose a control that invokes the preview-only activation command and shows the planned steps (labels and argv at minimum) without spawning activation processes. Preview presentation MAY reuse the activation results overlay shell with a clear preview mode indicator.

#### Scenario: User runs preview from UI

- **WHEN** the user triggers dry-run/preview for a valid profile from the hub or editor
- **THEN** the UI SHALL display planned steps from the preview command and SHALL NOT start activation child processes

# activation-preview Specification

## Purpose

Backend dry-run / preview of activation steps without executing them.

## Requirements

### Requirement: The system SHALL expose a preview-only activation command

The backend SHALL provide a command (e.g. `preview_session_activation`) that returns the ordered list of planned activation steps including `argv` (and optional `cwd`) for each step **without** spawning processes or performing irreversible side effects.

#### Scenario: Preview returns argv

- **WHEN** the client invokes preview for a valid profile
- **THEN** the response SHALL include each planned step with `argv` as it would be used by the real activation path

#### Scenario: Preview does not execute

- **WHEN** preview is invoked
- **THEN** no child processes for activation steps SHALL be started and no activation log SHALL be appended for real execution

### Requirement: Dry-run UI is out of scope

A dedicated dry-run UI (hub overflow, editor tab) SHALL NOT be required in the current release. The preview command exists for tooling and future UI.

#### Scenario: No dry-run button required

- **WHEN** the user views the session hub or profile editor
- **THEN** the UI SHALL NOT be required to expose dry-run controls

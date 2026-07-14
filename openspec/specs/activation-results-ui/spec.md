# activation-results-ui Specification

## Purpose

Backend activation reporting. In-app results UI (timeline overlay, open log) is **out of scope** for the current release; activation completes without a dedicated results panel.

## Requirements

### Requirement: Activation SHALL return structured step results

When the client invokes `activate_session_profile`, the backend SHALL return an `ActivateSessionResult` with per-step status (success, failure, skipped, warning), labels, and optional log file path. The client MAY use this data for future UI; displaying it is not required in the current build.

#### Scenario: Hub activation returns result

- **WHEN** activation is triggered from the session hub
- **THEN** the invoke SHALL complete with structured step results without requiring the UI to open a results overlay

#### Scenario: Failed step in result

- **WHEN** any activation step fails
- **THEN** the returned result SHALL mark that step as failed and the overall outcome SHALL reflect failure

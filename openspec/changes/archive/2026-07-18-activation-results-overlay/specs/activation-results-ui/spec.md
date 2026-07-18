## ADDED Requirements

### Requirement: Client SHALL present an activation results overlay

After `activate_session_profile` completes from the session hub or the profile editor (success or failure payload), the UI SHALL open a dismissible overlay that lists each returned step with its status and label (and detail when present). The overlay SHALL remain open until the user dismisses it. Discarding the result without presenting the overlay SHALL NOT satisfy this requirement.

#### Scenario: Hub activation opens overlay

- **WHEN** the user activates a session from the session hub and the invoke returns step results
- **THEN** the activation results overlay SHALL open and SHALL list those steps with visible status

#### Scenario: Editor activation opens overlay

- **WHEN** the user activates from the profile editor and the invoke returns step results
- **THEN** the activation results overlay SHALL open with the same step presentation rules as hub activation

#### Scenario: User dismisses overlay

- **WHEN** the results overlay is open and the user chooses dismiss (close control or equivalent)
- **THEN** the overlay SHALL close and the underlying hub or editor SHALL remain usable

### Requirement: Overlay SHALL reflect overall outcome and invoke errors

The overlay SHALL indicate whether activation overall succeeded or failed based on step statuses (any failed step means a failed overall outcome). When the activate invoke throws or otherwise fails before usable step results are available, the UI SHALL still present the overlay (or equivalent modal) with a clear error message so the failure is not silent.

#### Scenario: Failed step shows failed outcome

- **WHEN** the returned result includes at least one step with status failure
- **THEN** the overlay SHALL present a failed overall outcome and SHALL show that step as failed

#### Scenario: Invoke exception is visible

- **WHEN** activation is triggered and the client invoke fails with an error
- **THEN** the user SHALL see a dismissible error presentation and SHALL NOT be left without feedback

### Requirement: Overlay MAY offer open activation log

When the activation result includes a non-empty `activationLogPath` (or equivalent), the overlay SHALL offer an action to open that log file with the system handler. When no log path is present, the open-log action SHALL be hidden or disabled.

#### Scenario: Open log when path present

- **WHEN** the results overlay is shown and `activationLogPath` is present
- **THEN** the user SHALL be able to trigger open-log and the app SHALL attempt to open that file path

#### Scenario: No log path

- **WHEN** the results overlay is shown and no activation log path is returned
- **THEN** the UI SHALL NOT require a working open-log action

## MODIFIED Requirements

### Requirement: Activation SHALL return structured step results

When the client invokes `activate_session_profile`, the backend SHALL return an `ActivateSessionResult` with per-step status (success, failure, skipped, warning), labels, and optional log file path. The client SHALL use this data to present the activation results overlay required by this capability.

#### Scenario: Hub activation returns result

- **WHEN** activation is triggered from the session hub
- **THEN** the invoke SHALL complete with structured step results and the UI SHALL open the activation results overlay for those results

#### Scenario: Failed step in result

- **WHEN** any activation step fails
- **THEN** the returned result SHALL mark that step as failed and the overall outcome SHALL reflect failure

# context-cleanup Specification

## Purpose
TBD - created by archiving change maestro-mvp. Update Purpose after archive.
## Requirements
### Requirement: Linux process enumeration for diff

The system SHALL, on Linux, enumerate user-relevant running processes through a platform adapter (implementation detail) and derive a comparable executable identifier (for example basename of the executable path) for each candidate process.

#### Scenario: Enumerate excludes kernel-only noise

- **WHEN** the diff view is requested
- **THEN** the system SHALL not offer to terminate kernel threads or obvious system daemons from a built-in denylist

### Requirement: Divergence list vs active session

The system SHALL compare running processes against the set implied by the selected session profile (executables required or allowed by the profile and documented rules). The system SHALL present a list of divergent processes (running but not in the allowed set) to the user before any termination.

#### Scenario: No divergences

- **WHEN** every running user application matches the allowed set for the session
- **THEN** the system SHALL show an empty divergence list and SHALL not prompt for termination

### Requirement: Graceful termination with explicit consent

The system SHALL send SIGTERM to selected processes only after explicit user confirmation in the UI. The system SHALL send SIGKILL only if the user opts into force-kill and only after an optional configurable timeout following SIGTERM. The system SHALL never terminate without a confirmation action tied to the specific PID or process row.

#### Scenario: User confirms SIGTERM

- **WHEN** the user confirms termination for a listed PID
- **THEN** the system SHALL send SIGTERM to that PID and SHALL update the UI with success or failure

#### Scenario: User cancels

- **WHEN** the user dismisses the cleanup confirmation dialog
- **THEN** the system SHALL send no signals


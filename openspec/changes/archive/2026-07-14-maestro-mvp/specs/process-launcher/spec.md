## ADDED Requirements

### Requirement: Spawn application from profile entry

The system SHALL spawn a child process for each application launch entry using the configured executable path or name (resolved via system PATH when not absolute), the configured argument list, and optional current working directory. The system SHALL not block the UI thread indefinitely while spawning.

#### Scenario: Successful IDE launch with project path

- **WHEN** a launch entry specifies executable `cursor` and arguments include an absolute project directory
- **THEN** the system SHALL start the process with those arguments and SHALL record success in the activation summary

#### Scenario: Executable missing

- **WHEN** the configured executable cannot be resolved or started
- **THEN** the system SHALL record a failure for that entry with a human-readable message and SHALL continue or abort the activation plan according to session-activation policy

### Requirement: Sequential launch with optional delay

The system SHALL support launching multiple application entries in order. The system SHALL support an optional configurable delay between consecutive spawns to reduce startup spikes.

#### Scenario: Ordered launches

- **WHEN** a profile defines three application entries A, B, C in order and delay is set to 200ms
- **THEN** the system SHALL start A before B, B before C, and SHALL wait at least the configured delay between successful starts unless the delay is zero

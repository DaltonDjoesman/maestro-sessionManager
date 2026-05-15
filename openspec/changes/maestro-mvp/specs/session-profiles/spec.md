## ADDED Requirements

### Requirement: Session profile JSON schema

The system SHALL persist each session profile as a JSON document on the local filesystem. Each document SHALL include a `schema_version` field (positive integer) and a unique session identifier. The document SHALL support: human-readable session name; ordered list of application launch entries (executable, arguments array, optional working directory); optional browser launch block; optional rules for context cleanup.

#### Scenario: Load valid profile

- **WHEN** the user opens an existing profile file that matches the supported `schema_version`
- **THEN** the system SHALL deserialize the profile without data loss for supported fields

#### Scenario: Reject unknown major schema

- **WHEN** the profile `schema_version` is greater than the newest version supported by this application build
- **THEN** the system SHALL refuse to activate the profile and SHALL surface a clear error naming the file and version

### Requirement: Profile catalog CRUD

The system SHALL allow the user to create, read, update, duplicate, and delete session profiles through the UI. The system SHALL write changes atomically where practical (write-temp-then-rename) to reduce corruption risk. The user SHALL be able to edit the underlying JSON manually outside the app; on next load the system SHALL validate and report errors.

#### Scenario: Duplicate profile

- **WHEN** the user chooses duplicate on an existing profile
- **THEN** the system SHALL create a new profile file with a new unique identifier and SHALL not modify the source file until the user saves edits

#### Scenario: Manual JSON edit breaks syntax

- **WHEN** a profile file on disk contains invalid JSON
- **THEN** the system SHALL mark the profile as invalid in the catalog and SHALL show an error that includes the file path

### Requirement: Profiles storage location

The system SHALL store profiles under a configurable root directory (see application-settings). The system SHALL list all profile files in that directory for the catalog view.

#### Scenario: Empty profiles directory

- **WHEN** the profiles directory exists and contains no profile files
- **THEN** the system SHALL show an empty catalog and SHALL still allow creating a new profile

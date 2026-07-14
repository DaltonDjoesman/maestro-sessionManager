# profile-portability Specification

## Purpose
TBD - created by archiving change session-hub-activation-ux-export. Update Purpose after archive.
## Requirements
### Requirement: Users SHALL export a session profile to JSON

From the profile editor or catalog context menu, the user SHALL be able to export the current profile to a JSON file including `schema_version` and canonical fields.

#### Scenario: Export writes valid JSON

**WHEN** export is chosen for a profile  
**THEN** the written file SHALL parse as JSON and SHALL include `schema_version` equal to the profile’s version.

### Requirement: Users SHALL import a session profile from JSON

The app SHALL allow picking a JSON file, validating `schema_version` and required fields, then saving as a new profile file under the active `profiles_root` with a user-chosen name (subject to filename safety rules).

#### Scenario: Import rejects unknown schema

**WHEN** imported JSON has an unsupported `schema_version`  
**THEN** the app SHALL refuse import with a clear error and SHALL not write a partial profile file.

#### Scenario: Import succeeds

**WHEN** imported JSON matches a supported schema and the chosen name is unique under `profiles_root`  
**THEN** a new profile file SHALL appear and the catalog SHALL list it without restart.

### Requirement: Users SHALL duplicate a profile with a new name

The catalog or editor SHALL offer “Duplicar” prompting for a new display name (and derived safe filename), copying all profile fields.

#### Scenario: Duplicate creates independent copy

**WHEN** duplication succeeds  
**THEN** edits to the new profile SHALL not affect the original file until explicitly saved per profile lifecycle rules.


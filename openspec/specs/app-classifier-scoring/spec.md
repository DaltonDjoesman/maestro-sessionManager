# app-classifier-scoring Specification

## Purpose
TBD - created by archiving change universal-app-classification. Update Purpose after archive.
## Requirements
### Requirement: Scored app vs process classification

The system SHALL classify each running assistant candidate using a scored model. Classification SHALL NOT depend on a hardcoded allowlist of executable basenames as the primary mechanism. A candidate SHALL be classified as `app` when its score meets or exceeds the configured application threshold after noise exclusion. A candidate SHALL be classified as `process` when below the threshold.

#### Scenario: Window-backed app without hardcoded basename

- **WHEN** a running program has a mapped top-level window and no denylist match, even if its executable basename is not in any static allowlist
- **THEN** the candidate SHALL have `kind` equal to `app`

#### Scenario: Background daemon with desktop file but no window

- **WHEN** a process matches a `.desktop` entry but has no mapped top-level window and is not a known editor with special cwd handling
- **THEN** the candidate SHALL have `kind` equal to `process`

#### Scenario: Hardcoded basename allowlist is not required

- **WHEN** the build ships without a static GUI basename allowlist (or with an empty allowlist)
- **THEN** the assistant SHALL still classify deb-installed and Flatpak-installed GUI applications that have windows or strong `.desktop` matches as `app`

### Requirement: Classification confidence indicator

The system SHALL expose an optional `classificationConfidence` field with values `high`, `medium`, or `low` derived from whether classification used window mapping, `.desktop` matching, or neither.

#### Scenario: High confidence for window and desktop match

- **WHEN** a candidate has both a mapped window and a matched `.desktop` entry
- **THEN** `classificationConfidence` SHALL be `high`

#### Scenario: Medium confidence for window only

- **WHEN** a candidate has a mapped window but no `.desktop` match
- **THEN** `classificationConfidence` SHALL be `medium`


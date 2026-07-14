# activation-preview Specification

## Purpose
TBD - created by archiving change session-hub-activation-ux-export. Update Purpose after archive.
## Requirements
### Requirement: The system SHALL expose a preview-only activation command

The backend SHALL provide a command (e.g. `preview_session_activation`) that returns the ordered list of planned activation steps including `argv` (and optional `cwd`) for each step **without** spawning processes or performing irreversible side effects.

#### Scenario: Preview returns argv

**WHEN** the client invokes preview for a valid profile id  
**THEN** the response SHALL include each planned step with `argv` as it would be used by the real activation path.

#### Scenario: Preview does not execute

**WHEN** preview is invoked  
**THEN** no child processes for activation steps SHALL be started and no activation log SHALL be appended for real execution (preview may use a dedicated marker or omit logging entirely).

### Requirement: The activation UI SHALL offer dry-run before confirm

The user SHALL be able to open a dry-run preview showing argv per step. Dry-run SHALL be available from the editor **Avançado** tab and MAY be offered from the session hub card overflow menu.

#### Scenario: User reviews argv

- **WHEN** the user opens dry-run from the activation confirmation surface
- **THEN** the preview panel SHALL list all steps with argv visible before the user chooses **Ativar**

#### Scenario: Dry-run from Advanced tab

- **WHEN** the user opens dry-run from the editor Avançado tab
- **THEN** the preview panel SHALL list all steps with argv visible

#### Scenario: Dry-run from hub overflow

- **WHEN** the user chooses dry-run from a hub card overflow menu
- **THEN** the preview SHALL run against the on-disk profile without opening the editor


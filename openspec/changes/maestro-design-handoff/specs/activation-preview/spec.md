## MODIFIED Requirements

### Requirement: The activation UI SHALL offer dry-run before confirm

The user SHALL be able to open a dry-run preview showing argv per step. Dry-run SHALL be available from the editor **Avançado** tab and from the session hub card overflow menu. Dry-run output SHALL appear in the same terminal-style overlay used for activation results, with a clear dry-run indicator.

#### Scenario: Dry-run from Advanced tab

- **WHEN** the user opens dry-run from the editor Avançado tab
- **THEN** the terminal overlay SHALL list all steps with argv visible and indicate dry-run mode

#### Scenario: Dry-run from hub overflow

- **WHEN** the user chooses dry-run from a hub card overflow menu
- **THEN** the preview SHALL run against the on-disk profile without opening the editor and SHALL display in the terminal overlay

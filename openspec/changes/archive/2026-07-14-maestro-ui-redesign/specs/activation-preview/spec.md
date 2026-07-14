## MODIFIED Requirements

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

# session-editor-layout Specification

## Purpose
TBD - created by archiving change maestro-ui-redesign. Update Purpose after archive.
## Requirements
### Requirement: Profile editor SHALL use a fixed header and tabs

The profile editor SHALL show a fixed header with editable session name, breadcrumb back to Sessões, **Guardar**, and **Ativar**. Body content SHALL be organized into tabs: **Conteúdo** (applications + browser), **Captura** (running-apps assistant when enabled in settings), and **Avançado** (read-only session id, dry-run, technical hints).

#### Scenario: Session id in Advanced only

- **WHEN** the user opens the editor
- **THEN** `session_id` SHALL appear only under the **Avançado** tab

#### Scenario: Capture tab hidden when assistant disabled

- **WHEN** assisted profile capture is disabled in settings
- **THEN** the **Captura** tab SHALL be hidden or disabled with an explanation


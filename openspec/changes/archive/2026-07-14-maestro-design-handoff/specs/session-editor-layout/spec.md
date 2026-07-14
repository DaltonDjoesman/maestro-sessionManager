## MODIFIED Requirements

### Requirement: Profile editor SHALL use a fixed header and tabs

The profile editor SHALL show a fixed header with editable session name, breadcrumb back to Sessões, **Guardar**, and **Ativar** using prototype button styles. Body content SHALL be organized into tabs: **Conteúdo** (applications + browser), **Captura** (running-apps assistant when enabled in settings), and **Avançado** (read-only session id, dry-run, JSON editor).

Form sections SHALL use prototype section titles, spacing, and field layouts (general data, sequential applications list, browser toggle block).

#### Scenario: Session id in Advanced only

- **WHEN** the user opens the editor
- **THEN** `session_id` SHALL appear only under the **Avançado** tab

#### Scenario: Capture tab hidden when assistant disabled

- **WHEN** assisted profile capture is disabled in settings
- **THEN** the **Captura** tab SHALL be hidden or disabled with an explanation

#### Scenario: Sequential applications list styling

- **WHEN** the user views the **Conteúdo** tab with multiple applications
- **THEN** each application row SHALL match prototype list item layout including remove/add controls

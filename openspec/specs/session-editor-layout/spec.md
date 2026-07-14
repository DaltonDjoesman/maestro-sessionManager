# session-editor-layout Specification

## Purpose

Profile editor layout: header actions and tabbed body for content and capture.

## Requirements

### Requirement: Profile editor SHALL use a fixed header and tabs

The profile editor SHALL show a fixed header with editable session name, **Guardar**, and **Ativar** using prototype button styles. Body content SHALL be organized into tabs: **Conteúdo** (applications with per-app browser settings) and **Captura** (running-apps assistant).

Form sections SHALL use prototype section titles, spacing, and field layouts (sequential applications list).

#### Scenario: Capture tab always available

- **WHEN** the user opens the editor
- **THEN** the **Captura** tab SHALL be visible and usable

#### Scenario: Sequential applications list styling

- **WHEN** the user views the **Conteúdo** tab with multiple applications
- **THEN** each application row SHALL match prototype list item layout including remove/add controls

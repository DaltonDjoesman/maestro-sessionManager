# ui-localization Specification

## Purpose
TBD - created by archiving change maestro-ui-redesign. Update Purpose after archive.
## Requirements
### Requirement: User-facing strings SHALL be in Portuguese

Primary UI labels, buttons, section headings, and empty states SHALL be in European Portuguese. Strings SHALL be centralized (e.g. `src/i18n/pt.ts`) to avoid mixed-language UI. Copy SHALL follow prototype semantics but use pt-PT forms (e.g. **Guardar**, **Definições**, **Desactivar**, **Importar**).

#### Scenario: Hub actions in Portuguese

- **WHEN** the session hub is displayed
- **THEN** primary actions SHALL read **Ativar**, **Editar**, **Nova sessão**, and **Definições** (not English equivalents)

#### Scenario: Hub import in Portuguese

- **WHEN** the session hub is displayed
- **THEN** the import action SHALL read **Importar** (not English or pt-BR equivalents)

#### Scenario: Capture screen in Portuguese

- **WHEN** the standalone capture screen is displayed
- **THEN** headings and actions SHALL use pt-PT strings centralized in i18n


## MODIFIED Requirements

### Requirement: User-facing strings SHALL be in Portuguese

Primary UI labels, buttons, section headings, and empty states SHALL default to European Portuguese. Strings SHALL be centralized in an i18n module layout that supports multiple locale tables (at least `pt` and an `en` stub or complete table) and a single access path so new languages can be added without scattering literals. Copy SHALL follow prototype semantics but use pt-PT forms for the default locale (e.g. **Guardar**, **Definições**, **Importar**). Misleading teardown wording such as **Desactivar** for a label-only clear action SHALL be avoided in favor of launcher-honest copy.

#### Scenario: Hub actions in Portuguese

- **WHEN** the session hub is displayed with the default locale
- **THEN** primary actions SHALL read **Ativar**, **Editar**, **Nova sessão**, and **Definições** (not English equivalents)

#### Scenario: Hub import in Portuguese

- **WHEN** the session hub is displayed with the default locale
- **THEN** the import action SHALL read **Importar** (not English or pt-BR equivalents)

#### Scenario: Capture screen in Portuguese

- **WHEN** the standalone capture screen is displayed with the default locale
- **THEN** headings and actions SHALL use pt-PT strings centralized in i18n

#### Scenario: Additional locale module can be added

- **WHEN** a maintainer adds a new locale file following the i18n layout
- **THEN** UI string lookups SHALL be able to resolve keys from that locale without rewriting call sites that already use the shared accessor

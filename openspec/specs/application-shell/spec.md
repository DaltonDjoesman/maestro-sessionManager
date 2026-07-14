# application-shell Specification

## Purpose
TBD - created by archiving change maestro-ui-redesign. Update Purpose after archive.
## Requirements
### Requirement: Application SHALL use a persistent sidebar shell

The application SHALL render a persistent sidebar with **Sessões** and **Definições** as top-level destinations. **Sessões** SHALL be the default view on launch. There SHALL NOT be separate top-level **Home** or **About** navigation items.

#### Scenario: Launch opens session hub

- **WHEN** the application starts
- **THEN** the main content area SHALL show the session hub (not a separate welcome home)

#### Scenario: Narrow viewport uses drawer

- **WHEN** the viewport width is below 900px
- **THEN** the sidebar SHALL collapse into a drawer or overlay pattern without losing navigation to Sessões and Definições

### Requirement: Theme tokens SHALL apply to the redesigned shell

When `ApplicationSettings.theme` is set, the sidebar and main content SHALL use design tokens from `data-theme` on the document root.

#### Scenario: Sidebar uses theme tokens

- **WHEN** the application loads with a saved theme setting
- **THEN** the sidebar and main content area SHALL render using CSS variables tied to `data-theme` on the document root


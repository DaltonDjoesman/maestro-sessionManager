## MODIFIED Requirements

### Requirement: Application SHALL use a persistent sidebar shell

The application SHALL render a persistent sidebar with **Sessões**, **Captura**, and **Definições** as top-level destinations, plus **Editor** when a profile is being edited. **Sessões** SHALL be the default view on launch. There SHALL NOT be separate top-level **Home** or **About** navigation items.

The shell SHALL include a window header with application logo/title and a theme toggle control.

The sidebar footer SHALL show a **current session** widget with status indicator and session name when a session is active or was last activated.

#### Scenario: Launch opens session hub

- **WHEN** the application starts
- **THEN** the main content area SHALL show the session hub (not a separate welcome home)

#### Scenario: Capture route in sidebar

- **WHEN** the user clicks **Captura** in the sidebar
- **THEN** the standalone capture assistant screen SHALL open without opening the profile editor

#### Scenario: Narrow viewport uses drawer

- **WHEN** the viewport width is below 900px
- **THEN** the sidebar SHALL collapse into a drawer or overlay pattern without losing navigation to Sessões, Captura, and Definições

### Requirement: Theme tokens SHALL apply to the redesigned shell

When `ApplicationSettings.theme` is set, the sidebar, window header, and main content SHALL use OKLCH design tokens from the exported prototype via `data-theme` on the document root. A header theme toggle SHALL cycle or toggle between light and dark consistent with saved settings.

#### Scenario: Sidebar uses theme tokens

- **WHEN** the application loads with a saved theme setting
- **THEN** the sidebar, window header, and main content area SHALL render using CSS variables tied to `data-theme` on the document root

#### Scenario: Theme toggle in header

- **WHEN** the user clicks the theme toggle in the window header
- **THEN** the application theme SHALL update immediately and persist through settings save

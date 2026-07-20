## MODIFIED Requirements

### Requirement: Application SHALL use a persistent sidebar shell

The application SHALL render a persistent sidebar with **Sessões**, **Captura**, and **Definições** as top-level destinations, plus **Editor** when a profile is being edited. **Sessões** SHALL be the default view on launch. There SHALL NOT be separate top-level **Home** or **About** navigation items.

The shell SHALL include a window header with application logo/title and a theme toggle control.

The sidebar footer MAY show a **last activated session** indicator (name/status) for convenience. Maestro is a session **launcher**: the footer MUST NOT imply that processes are managed or that a “deactivate” action tears down launched apps. Any clear-label control SHALL only clear the local indicator (or equivalent UI state), not kill or close applications.

#### Scenario: Launch opens session hub

- **WHEN** the application starts
- **THEN** the main content area SHALL show the session hub (not a separate welcome home)

#### Scenario: Capture route in sidebar

- **WHEN** the user clicks **Captura** in the sidebar
- **THEN** the standalone capture assistant screen SHALL open without opening the profile editor

#### Scenario: Narrow viewport uses drawer

- **WHEN** the viewport width is below 900px
- **THEN** the sidebar SHALL collapse into a drawer or overlay pattern without losing navigation to Sessões, Captura, and Definições

#### Scenario: Footer does not tear down session

- **WHEN** the user clears the last-activated indicator from the sidebar footer
- **THEN** the application SHALL update only local UI state and SHALL NOT terminate processes launched by activation

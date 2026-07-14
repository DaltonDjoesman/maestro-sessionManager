## MODIFIED Requirements

### Requirement: Running applications suggestion list

The system SHALL provide an assistant that lists candidate user applications currently running. When enabled in settings, the assistant SHALL be available from:

1. A dedicated **Captura** sidebar route (primary entry), and
2. The profile editor **Captura** tab for contextual capture while editing a profile.

#### Scenario: Standalone capture screen

- **WHEN** assisted capture is enabled and the user navigates to **Captura** in the sidebar
- **THEN** the running-apps list, filters, and create-from-capture actions SHALL be presented as a full-screen view matching the prototype

#### Scenario: Assistant in Capture tab

- **WHEN** assisted capture is enabled and the user opens the **Captura** tab in the editor
- **THEN** the running-apps list and selection UI SHALL be available there for the profile being edited

#### Scenario: Capture disabled

- **WHEN** assisted capture is disabled in settings
- **THEN** the **Captura** sidebar item SHALL be disabled or show guidance to enable it in Definições

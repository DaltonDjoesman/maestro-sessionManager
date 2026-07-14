## MODIFIED Requirements

### Requirement: Session hub SHALL be the unified catalog surface

The session hub SHALL replace separate Home and Sessions screens. It SHALL list profiles as compact cards styled per the exported prototype (not a data table), grouped into sections: **Continuar** (last session when valid), **Fixadas** (pinned), and **Todas as sessões**.

The hub SHALL expose a toolbar with search, **Importar**, and **Nova sessão** actions matching prototype layout and spacing.

#### Scenario: Continue last session section

- **WHEN** local UI state contains a valid last-opened profile path
- **THEN** the hub SHALL show a prominent continue card with **Ativar** and **Editar** actions

#### Scenario: Pinned section

- **WHEN** the user has pinned one or more profiles
- **THEN** pinned profiles SHALL appear in a **Fixadas** section before unpinned profiles in **Todas as sessões**

#### Scenario: Import via modal

- **WHEN** the user clicks **Importar** in the hub toolbar
- **THEN** an import modal styled per the prototype SHALL open for JSON paste or file selection

### Requirement: Session hub SHALL offer one-click activation

Each valid profile card SHALL expose **Ativar** as the primary action styled as a prototype primary/secondary button pair. Activation SHALL load the profile JSON from disk and invoke activation without requiring the editor to open.

#### Scenario: Activate from card

- **WHEN** the user clicks **Ativar** on a valid profile card
- **THEN** the system SHALL activate using the on-disk profile and SHALL open the terminal-style activation overlay

### Requirement: Session hub SHALL show human-readable summaries

Each profile card SHALL show a summary line derived from metadata (e.g. app count, browser presence, URL count) and contextual badges (`browser_only`, invalid) using prototype badge styling.

#### Scenario: Summary line

- **WHEN** a profile has 2 applications and a browser block with 3 URLs
- **THEN** the card summary SHALL indicate apps and browser content in plain language

### Requirement: Secondary actions SHALL use overflow menu

Duplicate, export, delete, and dry-run SHALL be available from a per-card overflow menu (**···**) styled per prototype, not as always-visible row buttons.

#### Scenario: Overflow menu exposes secondary actions

- **WHEN** the user opens the overflow menu on a profile card
- **THEN** duplicate, export, delete, and dry-run actions SHALL be available without always-visible row buttons on the card

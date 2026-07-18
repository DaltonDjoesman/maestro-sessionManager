# session-hub Specification

## Purpose

Unified session catalog (hub) for listing, activating, and managing profiles.

## Requirements

### Requirement: Session hub SHALL be the unified catalog surface

The session hub SHALL replace separate Home and Sessions screens. It SHALL list profiles as compact cards styled per the exported prototype (not a data table), grouped into sections: **Continuar** (last session when valid), **Fixadas** (pinned), and **Todas as sessões**.

The hub SHALL expose a toolbar with search, **Importar**, and **Nova sessão** actions matching prototype layout and spacing.

#### Scenario: Continue last session section

- **WHEN** local UI state contains a valid last-opened profile path
- **THEN** the hub SHALL show a prominent continue card with **Ativar**; clicking the card body opens the editor

#### Scenario: Pinned section

- **WHEN** the user has pinned one or more profiles
- **THEN** pinned profiles SHALL appear in a **Fixadas** section before unpinned profiles in **Todas as sessões**

#### Scenario: Import via file picker

- **WHEN** the user clicks **Importar** in the hub toolbar
- **THEN** the system SHALL allow selecting a `.json` profile file and importing it with a display name prompt

### Requirement: Session hub SHALL offer one-click activation

Each valid profile card SHALL expose **Ativar** as the primary action styled as a prototype primary/secondary button pair. Activation SHALL load the profile JSON from disk and invoke activation without requiring the editor to open.

#### Scenario: Activate from card

- **WHEN** the user clicks **Ativar** on a valid profile card
- **THEN** the system SHALL activate using the on-disk profile and SHALL open the activation results overlay required by `activation-results-ui`

### Requirement: Session hub SHALL show human-readable summaries

Each profile card SHALL show metadata-derived badges (app count, browser presence, invalid state) using prototype badge styling.

#### Scenario: Summary badges

- **WHEN** a profile has applications and optional browser URLs
- **THEN** the card SHALL show badge hints for apps and browser content

### Requirement: Secondary actions SHALL use overflow menu

Duplicate, export, and delete SHALL be available from a per-card overflow menu (**···**) styled per prototype, not as always-visible row buttons.

#### Scenario: Overflow menu exposes secondary actions

- **WHEN** the user opens the overflow menu on a profile card
- **THEN** duplicate, export, and delete actions SHALL be available without always-visible row buttons on the card

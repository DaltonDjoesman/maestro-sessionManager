## ADDED Requirements

### Requirement: Empty hub SHALL guide first-time users

When the profiles catalog is empty (no profiles under the active `profiles_root`), the session hub SHALL show an empty state that explains what a session profile is and offers a primary path to create a profile and/or use the documented example profile (`docs/examples/` or an in-app “create from example” action). The empty state SHALL NOT leave the user with only a blank list.

#### Scenario: Empty profiles root

- **WHEN** the user opens the session hub and no profiles are listed
- **THEN** the hub SHALL show guidance plus at least one action to create a session or start from the example profile

## MODIFIED Requirements

### Requirement: Session hub SHALL be the unified catalog surface

The session hub SHALL replace separate Home and Sessions screens. It SHALL list profiles as compact cards styled per the exported prototype (not a data table), grouped into sections: **Continuar** (last session when valid), **Fixadas** (pinned), and **Todas as sessões**.

The hub SHALL expose a toolbar with search, **Importar**, and **Nova sessão** actions matching prototype layout and spacing. **Importar** SHALL use a dedicated import UI (modal or equivalent) for display name and confirmation rather than relying solely on a raw `window.prompt`.

#### Scenario: Continue last session section

- **WHEN** local UI state contains a valid last-opened profile path
- **THEN** the hub SHALL show a prominent continue card with **Ativar**; clicking the card body opens the editor

#### Scenario: Pinned section

- **WHEN** the user has pinned one or more profiles
- **THEN** pinned profiles SHALL appear in a **Fixadas** section before unpinned profiles in **Todas as sessões**

#### Scenario: Import via dedicated UI

- **WHEN** the user clicks **Importar** in the hub toolbar
- **THEN** the system SHALL allow selecting a `.json` profile file and importing it through a dedicated import UI with a display name field

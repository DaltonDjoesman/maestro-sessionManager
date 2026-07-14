## MODIFIED Requirements

### Requirement: Session hub SHALL surface last session and pinned profiles

The session hub (not a separate home screen) SHALL offer “Continuar última sessão” when a last-opened profile id exists in local UI state, and SHALL list pinned profiles in a dedicated section with stable order within pins.

#### Scenario: Continue last session visible

- **WHEN** the user has previously opened a profile and local UI state contains its id
- **THEN** the session hub SHALL show a primary action to activate or edit that profile

#### Scenario: Pinned profiles ordered first

- **WHEN** the user pins one or more profiles
- **THEN** those profiles SHALL appear before unpinned profiles in the catalog list

#### Scenario: Activate without opening editor

- **WHEN** the user activates from the hub card
- **THEN** activation SHALL complete without navigating to the profile editor

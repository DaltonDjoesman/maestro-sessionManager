## ADDED Requirements

### Requirement: Session hub SHALL surface last session and pinned profiles

The session catalog SHALL offer “Continuar última sessão” when a last-opened profile id exists in local UI state, and SHALL list pinned profiles first (stable order within pins).

#### Scenario: Continue last session visible

**WHEN** the user has previously opened a profile and local UI state contains its id  
**THEN** the home screen SHALL show a primary action to open that profile’s editor or activation flow (same as opening from the list).

#### Scenario: Pinned profiles ordered first

**WHEN** the user pins one or more profiles  
**THEN** those profiles SHALL appear before unpinned profiles in the catalog list.

### Requirement: Session catalog SHALL support search and contextual badges

The catalog SHALL filter profiles by display name (case-insensitive substring) and SHALL show badges derived from profile metadata: `browser_only`, and a count of `apps` when greater than zero.

#### Scenario: Search by name

**WHEN** the user types text in the catalog search field  
**THEN** only profiles whose name matches the substring SHALL remain visible.

#### Scenario: Browser-only badge

**WHEN** a profile has `browser_only` true  
**THEN** the catalog SHALL show a “Browser only” badge on that row.

#### Scenario: App count badge

**WHEN** a profile lists N app ids with N > 0  
**THEN** the catalog SHALL show a badge indicating N apps.

### Requirement: Pins and last session SHALL persist in browser local storage

Pins and last-opened profile id SHALL persist in `localStorage` keyed by a short hash of `profiles_root` so switching roots does not leak state.

#### Scenario: Switch profiles root clears irrelevant pins

**WHEN** `profiles_root` changes to a different directory  
**THEN** pin state for the previous root SHALL not apply to the new root’s catalog.

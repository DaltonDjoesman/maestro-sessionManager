## ADDED Requirements

### Requirement: Dry-run UI SHALL be available from hub or editor

The session hub and/or profile editor SHALL expose a control that invokes the preview-only activation command and shows the planned steps (labels and argv at minimum) without spawning activation processes. Preview presentation MAY reuse the activation results overlay shell with a clear preview mode indicator.

#### Scenario: User runs preview from UI

- **WHEN** the user triggers dry-run/preview for a valid profile from the hub or editor
- **THEN** the UI SHALL display planned steps from the preview command and SHALL NOT start activation child processes

## REMOVED Requirements

### Requirement: Dry-run UI is out of scope

**Reason:** Portfolio and trust goals require users to inspect planned launches before activating; the preview backend command already exists.

**Migration:** Implement preview controls per the ADDED dry-run UI requirement; keep `preview_session_activation` (or equivalent) as the source of planned argv.

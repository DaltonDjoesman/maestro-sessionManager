## ADDED Requirements

### Requirement: Activation SHALL support a preview mode separate from execution

The activation subsystem SHALL expose preview building of steps so UI and automated tests can assert parity between preview and executed argv without running commands.

#### Scenario: Preview parity with execute argv

**WHEN** the same profile is previewed and then executed (without mid-flight edits)  
**THEN** each step’s `argv` in preview SHALL match the argv used at spawn time for that step index, modulo intentional redaction documented elsewhere (none by default).

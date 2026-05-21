## ADDED Requirements

### Requirement: Superpowers skills for backend implementation

Any implementation work that primarily modifies Rust code under `src-tauri/` (including Tauri commands, `src-tauri/src/**` modules, and `Cargo.toml` / `tauri.conf.json` when behavior-relevant) SHALL follow the Superpowers skill **test-driven-development** before adding or changing production behavior, unless the OpenSpec task explicitly documents a justified exception.

#### Scenario: New Rust module behavior

- **WHEN** an agent or contributor adds or changes behavior in `src-tauri/src/`
- **THEN** they SHALL add or update unit tests in the same crate that fail before the change and pass after, and they SHALL run `cargo test` (or a scoped `cargo test <filter>`) before claiming completion

### Requirement: Verification before completion claims

Before stating that backend work is complete, fixed, or passing, the implementer SHALL run the verification commands appropriate to the change (at minimum `cargo test` for Rust-only changes, plus `npm run build` when the frontend or Tauri bridge types are touched) and SHALL align with the Superpowers skill **verification-before-completion**.

#### Scenario: User-visible success claim

- **WHEN** an implementer reports that a backend task is done or that tests pass
- **THEN** they SHALL have executed the documented verification commands in the same session and reflected failures before making the claim

### Requirement: Small descriptive Git commits

While implementing OpenSpec tasks that touch `src-tauri/`, the implementer SHALL make small, focused Git commits with clear messages (preferably conventional: `type(scope): imperative summary`) after each completed sub-task or logical unit, rather than one large commit at the end of a section.

#### Scenario: Multiple checkboxes in one session

- **WHEN** two or more distinct sub-tasks from `tasks.md` are completed in one working session
- **THEN** they SHALL create at least one commit per completed sub-task when practical, each with a message that states what changed and why

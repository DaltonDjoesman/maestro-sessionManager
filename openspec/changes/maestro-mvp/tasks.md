## 1. Project scaffold

- [x] 1.1 Initialise Tauri + Rust project in repo (src-tauri, frontend shell) with chosen UI framework
- [x] 1.2 Add core dependencies: serde/serde_json, tokio (if async process), sysinfo (or chosen Linux process crate)
- [x] 1.3 Define Rust modules: `platform` (Linux adapter trait), `profiles`, `settings`, `activation`, `browser`, `cleanup`

## 2. Application settings

- [x] 2.1 Implement global settings model with `schema_version` and default per-user data paths
- [ ] 2.2 Persist settings to disk; load on startup per application-settings spec
- [ ] 2.3 Build settings UI with validation for profiles root (exists, directory, writable)

## 3. Session profiles

- [ ] 3.1 Define Rust types matching session-profiles JSON (applications array, browser block, optional cleanup rules)
- [ ] 3.2 Implement profile discovery and catalog listing from configured directory
- [ ] 3.3 Implement CRUD + duplicate with atomic write (temp + rename) where practical
- [ ] 3.4 Validate on load; surface errors for invalid JSON or unsupported `schema_version`

## 4. Process launcher

- [ ] 4.1 Implement spawn helper: executable resolution, args, optional cwd, non-blocking error capture
- [ ] 4.2 Implement ordered launch with configurable inter-spawn delay
- [ ] 4.3 Map spawn results into activation summary entries (success/failure)

## 5. Browser launch

- [ ] 5.1 Implement command builder for `chromium-like` (`--new-window`, optional `--user-data-dir`, trailing URLs)
- [ ] 5.2 Implement command builder for `firefox` (`-new-window`, profile, optional `-no-remote`, trailing URLs)
- [ ] 5.3 Emit warnings when isolation fields empty (Chromium) or when `file://` URLs fail; keep HTTPS success path

## 6. Session activation

- [ ] 6.1 Implement validate-profile pipeline before any spawn
- [ ] 6.2 Orchestrate browser step then application steps (or documented order); return structured summary to UI
- [ ] 6.3 Optionally implement skip-if-already-running policy and reflect as `skipped` in summary

## 7. Context cleanup (Linux)

- [ ] 7.1 Implement Linux process enumeration behind platform adapter with denylist for system processes
- [ ] 7.2 Implement divergence computation vs profile-allowed executables
- [ ] 7.3 Build confirmation UI; send SIGTERM on confirm; optional timeout + SIGKILL only with explicit opt-in

## 8. UI integration

- [ ] 8.1 Session catalog view: list, create, edit, duplicate, delete profiles
- [ ] 8.2 Profile editor: applications table, browser block (family, executable, isolation fields, URL list)
- [ ] 8.3 Activate button wired to Tauri command returning step summary; display errors and warnings
- [ ] 8.4 Cleanup flow: show divergences, confirm, show outcome

## 9. Packaging and documentation

- [ ] 9.1 Document profile JSON schema and example files in repository README or `docs/`
- [ ] 9.2 Add Linux build instructions and target artifact (deb or AppImage or binary) for Pop!_OS
- [ ] 9.3 Smoke-test: activate sample profile with mock browser path in CI or manual checklist

## 10. Assisted profile capture (post-MVP slice)

- [ ] 10.1 Gate feature with config/flag per assisted-profile-capture spec
- [ ] 10.2 Implement running-apps candidate list with noise filter
- [ ] 10.3 Optional Linux cwd hint for known editors; merge selected rows into draft profile

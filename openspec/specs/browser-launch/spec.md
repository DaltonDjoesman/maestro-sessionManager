# browser-launch Specification

## Purpose
TBD - created by archiving change maestro-mvp. Update Purpose after archive.
## Requirements
### Requirement: Browser family command construction

The system SHALL build an OS process command line for launching a graphical browser based on a declared family: `chromium-like` or `firefox`. The profile SHALL specify the browser executable (absolute path or PATH-resolved name). The system SHALL append all configured URLs as trailing arguments after required flags.

#### Scenario: Chromium-like new window with multiple HTTPS tabs

- **WHEN** the profile browser block uses family `chromium-like`, includes flag `--new-window`, optional `--user-data-dir` pointing to a dedicated directory, and a non-empty URL list of HTTPS URLs
- **THEN** the system SHALL invoke the executable with those flags followed by the URLs such that one browser window opens with one tab per URL under supported configurations

#### Scenario: Firefox new window with profile isolation

- **WHEN** the profile browser block uses family `firefox`, includes `-new-window`, a dedicated profile name or path, and `-no-remote` when required by the profile
- **THEN** the system SHALL invoke Firefox with those options followed by the URL list

### Requirement: Isolation recommendation

The system SHALL document in-product or in bundled docs that for predictable session windows the user SHOULD configure Chromium `--user-data-dir` or a dedicated Firefox profile. The UI SHOULD expose fields for these isolation settings.

#### Scenario: User omits user-data-dir for Chromium

- **WHEN** the user activates a session with Chromium-like browser and empty user-data-dir
- **THEN** the system SHALL still attempt launch and SHALL attach a non-blocking warning that tabs may merge with an existing personal browser instance

### Requirement: file URL handling

The system SHALL allow `file://` URLs in the profile URL list. When launch fails or the OS/browser sandbox blocks file access, the system SHALL surface a warning and SHALL not treat HTTPS launch success as a failure of the whole browser step.

#### Scenario: file URL blocked

- **WHEN** at least one `file://` URL is configured and the browser process exits immediately with non-zero status or the activation reports file access denied
- **THEN** the system SHALL include a warning in the activation summary describing the file URL limitation


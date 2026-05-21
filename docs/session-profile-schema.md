# Session profile JSON (schema v1)

Maestro stores each **session profile** as a single UTF-8 JSON file under the configured profiles directory (see Settings). Files must use the `.json` extension to appear in the catalog.

## Top-level object

| Field | Type | Required | Notes |
|--------|------|----------|--------|
| `schema_version` | number | yes | Must be `1` for this build. |
| `session_id` | string | yes | Non-empty stable id (UUID recommended). Used for new filenames on create/duplicate. |
| `name` | string | yes | Human-readable session name. |
| `applications` | array | no (default `[]`) | Ordered list of app launches (see below). |
| `browser` | object or `null` | no | When set, activation runs the browser block before applications. |
| `cleanup` | object or `null` | no | Optional rules for divergence / cleanup (see below). |

## `applications[]` entries

| Field | Type | Required | Notes |
|--------|------|----------|--------|
| `executable` | string | yes | Path or name resolved like a shell command (non-empty after trim). |
| `args` | string[] | no | Default `[]`. Passed after the executable. |
| `cwd` | string or `null` | no | Working directory for the child process. |
| `skip_if_running` | boolean or `null` | no | When `true`, activation skips spawn if a process with the same **executable basename** is already running; summary status `skipped`. |

## `browser` block

| Field | Type | Required | Notes |
|--------|------|----------|--------|
| `family` | string | yes | `"chromium_like"` or `"firefox"`. |
| `executable` | string | yes | Browser binary (non-empty). |
| `user_data_dir` | string or `null` | no | Chromium-like: `--user-data-dir=…` when non-empty. |
| `firefox_profile` | string or `null` | no | Firefox: `-P` profile name when non-empty. |
| `firefox_no_remote` | boolean or `null` | no | Firefox: add `-no-remote` when `true`. |
| `urls` | string[] | no | Opened after flags; may include `https://` or `file://` (see browser-launch spec warnings). |

## `cleanup` rules

| Field | Type | Required | Notes |
|--------|------|----------|--------|
| `allow_extra_basenames` | string[] | no | Extra executable basenames treated as allowed when computing cleanup divergences. |

## Validation

On load and before save, the app validates `schema_version`, ids, non-empty names, non-empty application executables, and non-empty browser executable when `browser` is present. Unsupported `schema_version` values are rejected with a clear error.

## Examples

See [`docs/examples/smoke-session.profile.json`](examples/smoke-session.profile.json) for a minimal profile suitable for smoke testing activation with `/bin/true` as a stand-in “browser”.

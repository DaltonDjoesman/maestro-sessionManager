# Maestro

Desktop app (Tauri 2 + React + Rust) for orchestrating Linux work sessions: JSON profiles, app/browser launch, and activation summaries.

## Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started)
- [Node.js](https://nodejs.org/) (LTS or current)
- Linux desktop deps: [Tauri prerequisites](https://tauri.app/start/prerequisites/)

  On Pop!_OS / Ubuntu:

  ```bash
  sudo apt update
  sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
    libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
  ```

## Development

```bash
npm install
npm run tauri dev
```

## Session profiles (JSON)

Field-level documentation and validation rules: **[docs/session-profile-schema.md](docs/session-profile-schema.md)**.  
Example file for manual smoke testing: **[docs/examples/smoke-session.profile.json](docs/examples/smoke-session.profile.json)**.

## Linux release build (Pop!_OS / Ubuntu)

Prerequisites match [Tauri’s Linux setup](https://tauri.app/start/prerequisites/) (WebKitGTK, build tools, etc.).

From the repository root:

```bash
npm install
npm run tauri build
```

Artifacts are written under `src-tauri/target/release/bundle/`, for example:

- **Debian package:** `src-tauri/target/release/bundle/deb/` (`.deb` for the configured `identifier`).
- **AppImage:** `src-tauri/target/release/bundle/appimage/` when that target is produced for your toolchain.

The exact set depends on Tauri bundler support on your host; `bundle.targets` in `src-tauri/tauri.conf.json` is currently `"all"` (adjust if you want only `deb` or `appimage`).

## Smoke testing

Manual checklist (activation with mock `/bin/true` browser, catalog, settings): **[docs/smoke-test-checklist.md](docs/smoke-test-checklist.md)**.

## Backend / agent workflow

Maestro’s Rust core lives under `src-tauri/`. When implementing or reviewing backend work, follow **[docs/backend-workflow.md](docs/backend-workflow.md)** (Superpowers TDD, verification before completion, `cargo test`, small commits). The normative requirements live in **[openspec/specs/backend-dev-workflow/spec.md](openspec/specs/backend-dev-workflow/spec.md)**.

## Project layout

| Path | Role |
|------|------|
| `src/` | React UI (Vite) |
| `src-tauri/` | Rust core: profiles, settings, activation, browser, `platform` |
| `docs/session-profile-schema.md` | Session profile JSON fields (schema v1) |
| `docs/examples/` | Example profile JSON for smoke tests |
| `docs/smoke-test-checklist.md` | Manual QA checklist |
| `openspec/specs/backend-dev-workflow/` | OpenSpec capability: backend dev workflow (agents, TDD, commits) |

OpenSpec workflow commands live under `.cursor/commands/` (`opsx-apply`, etc.).

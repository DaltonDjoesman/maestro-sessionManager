# Maestro

Desktop app (Tauri 2 + React + Rust) for orchestrating Linux work sessions: JSON profiles, app/browser launch, optional process cleanup.

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

## Backend / agent workflow

Maestro’s Rust core lives under `src-tauri/`. When implementing or reviewing backend work, follow **[docs/backend-workflow.md](docs/backend-workflow.md)** (Superpowers TDD, verification before completion, `cargo test`, small commits). The normative OpenSpec change is **[openspec/changes/backend-superpowers-workflow/](openspec/changes/backend-superpowers-workflow/)**.

## Project layout

| Path | Role |
|------|------|
| `src/` | React UI (Vite) |
| `src-tauri/` | Rust core: profiles, settings, activation, browser, cleanup, `platform` |
| `openspec/changes/maestro-mvp/` | OpenSpec change: proposal, design, specs, tasks |
| `openspec/changes/backend-superpowers-workflow/` | OpenSpec change: backend dev workflow (agents, TDD, commits) |

OpenSpec workflow commands live under `.cursor/commands/` (`opsx-apply`, etc.).

# Contributing

Maestro is a **personal portfolio project**, not a commercial product or large community codebase. Contributions are welcome when they stay small and aligned with the launcher-only scope (see the README non-goals).

## How to help

1. **Issues** — Bug reports and focused feature ideas are fine. Include OS/desktop (e.g. Pop!_OS + Cosmic), steps to reproduce, and whether you used `tauri dev` or a release build.
2. **Pull requests** — Prefer narrow fixes (docs, copy, tests, small UI polish). Open an issue first for larger behavior changes.
3. **Scope** — Do not add session teardown/process kill, window placement, or cloud accounts unless discussed first.

## Local checks

```bash
npm install
npm run build
npm test
cd src-tauri && cargo test
```

Manual paths: [docs/smoke-test-checklist.md](docs/smoke-test-checklist.md).

System overview: [docs/architecture.md](docs/architecture.md). Publishing installers: [docs/releasing.md](docs/releasing.md).

## Security

Profiles and settings are local JSON only — no cloud accounts. If you discover a vulnerability in how profiles or IPC are handled, open a private GitHub security advisory (or an issue without exploit detail) rather than posting a public PoC.

By contributing, you agree your changes are licensed under the same [MIT License](LICENSE) as the project.

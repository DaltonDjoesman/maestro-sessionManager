## 1. Documentation

- [x] 1.1 Add `docs/backend-workflow.md` listing Superpowers skill paths (TDD, verification-before-completion, systematic-debugging), scope `src-tauri/`, and verification commands (`cargo test`, `npm run build` when UI/types change)
- [ ] 1.2 Update root `README.md` with a "Backend / agent workflow" link to that doc and to `openspec/changes/backend-superpowers-workflow/`

## 2. Cursor guidance

- [ ] 2.1 Add `.cursor/rules/maestro-backend.mdc` (alwaysApply or globs on `src-tauri/**`) pointing agents to `docs/backend-workflow.md` and the OpenSpec spec `backend-dev-workflow`

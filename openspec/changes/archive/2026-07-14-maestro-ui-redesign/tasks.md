# Tasks: maestro-ui-redesign

## 1. OpenSpec & docs

- [x] 1.1 Create change scaffold (proposal, design, specs).
- [x] 1.2 Update `docs/smoke-test-checklist.md` for sidebar hub flow.

## 2. Foundation (Fase A)

- [x] 2.1 Add `src/styles/tokens.css`, `themes.css`, and `src/i18n/pt.ts`.
- [x] 2.2 Add `AppShell` + `Sidebar` layout; wire routes in `App.tsx`.
- [x] 2.3 Group settings sections; absorb About footer.

## 3. Session hub (Fase B)

- [x] 3.1 Implement `SessionHubPage` with compact card list and sections.
- [x] 3.2 Add hub **Ativar** (load + activate); shared `ActivationResultsPanel` modal.
- [x] 3.3 Overflow menu: duplicate, export, delete, dry-run.

## 4. Editor (Fase C)

- [x] 4.1 Refactor editor: fixed header, tabs Conteúdo / Captura / Avançado.
- [x] 4.2 Wire editor activation to shared results panel.
- [x] 4.3 Move dry-run and session_id to Avançado tab.

## 5. Polish (Fase D)

- [x] 5.1 Responsive sidebar drawer; empty states; PT strings throughout.
- [x] 5.2 Run `npm run build` and `cargo test` verification.

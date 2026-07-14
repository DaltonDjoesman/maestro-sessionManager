## Context

React + Tauri 2. Navegação actual em [App.tsx](../../../src/App.tsx) com top bar (Home, Sessions, Settings, About). Catálogo em tabela; editor monolítico.

## Goals / Non-Goals

**Goals:**
- Hub-first: abrir app → ver sessões → ativar ou editar.
- Sidebar persistente; About absorvido em Definições.
- Componentes partilhados: `ActivationResultsPanel`, tokens CSS.
- PT como língua da UI.

**Non-Goals:**
- Alterar schema JSON de perfis.
- Novo comando backend obrigatório (MVP: load + activate no frontend).
- Cloud sync.

## Decisions

| Decisão | Escolha |
|---------|---------|
| Activar no hub | Directo, sem confirmação |
| Densidade hub | Lista compacta |
| Pins / última sessão | Manter `localStorage` em [sessionCatalogUi.ts](../../../src/sessionCatalogUi.ts) |
| Resultados activação | Modal controlado em `App.tsx` |
| Rotas | Estado `hub \| editor \| settings` + `editorPath` |

## File layout

```
src/layout/AppShell.tsx, Sidebar.tsx
src/pages/SessionHubPage.tsx
src/components/ProfileEditorScreen.tsx  (tabs)
src/components/SettingsScreen.tsx       (grouped)
src/components/ActivationResultsPanel.tsx
src/i18n/pt.ts
src/styles/tokens.css, themes.css
```

## Risks

| Risco | Mitigação |
|--------|-----------|
| Editor refactor grande | Tabs sem alterar lógica de save/activate |
| CSS regressão | Manter classes existentes onde possível; tokens novos |

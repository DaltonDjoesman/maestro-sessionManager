## Why

O Maestro já entrega catálogo com pins, export/import e activação com timeline, mas a **navegação** ainda separa Home e Sessions, a **activação** só está no fundo do editor, e a UI mistura informação técnica com acções do dia-a-dia. Utilizadores precisam de **gerir a biblioteca de perfis**, **ativar em 1 clique** e **editar/criar** sem fricção.

Pré-requisito implementado: [session-hub-activation-ux-export](../session-hub-activation-ux-export/proposal.md) (pins, badges, dry-run, timeline, tema).

## What Changes

- **Shell com sidebar**: Sessões (default) e Definições; sem abas Home/About no topo; About no rodapé de Definições.
- **Hub de sessões único**: fundir Home + catálogo; lista compacta em cards; secções Continuar / Fixadas / Todas.
- **Ativar no hub**: acção primária no card sem abrir o editor (perfil carregado do disco).
- **Resultados partilhados**: modal/sheet de timeline acessível desde hub e editor.
- **Editor em tabs**: Conteúdo · Captura · Avançado; header fixo com Guardar e Ativar.
- **Definições agrupadas**: Geral, Sessões, Browser, Assistente, Avançado.
- **UI em português**: strings centralizadas em `src/i18n/pt.ts`.
- **Design system**: tokens CSS, sidebar, cards.

**Decisões de produto (fechadas):**
- Ativar no hub **sem** diálogo de confirmação (activação directa com estado de carregamento).
- Layout do hub: **lista compacta** (não grelha).

## Capabilities

### New Capabilities

- `application-shell`: Sidebar, rotas, drawer responsivo.
- `session-hub`: Hub unificado com activação 1-clique e resumo por sessão.
- `session-editor-layout`: Editor com tabs e header fixo.
- `ui-localization`: Interface em PT.

### Modified Capabilities

- `session-catalog-ux`: Hub como ecrã principal; activar sem editor.
- `activation-results-ui`: Painel partilhado hub + editor.
- `activation-preview`: Dry-run em Avançado e menu overflow.
- `application-settings`: Secções agrupadas; About no rodapé.
- `assisted-profile-capture`: Assistant na tab Captura.

## Impact

- **Frontend**: `App.tsx`, novo layout/pages/components, `App.css`, i18n.
- **Backend**: sem breaking changes; activação rápida via `load_session_profile` + `activate_session_profile`.
- **Docs**: `docs/smoke-test-checklist.md` actualizado.

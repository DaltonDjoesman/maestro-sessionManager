## Why

O Maestro já tem a arquitectura UX correcta (hub, sidebar, editor em tabs, PT), mas a interface actual usa tokens hex genéricos e componentes CSS ad-hoc que não reflectem o design exportado em `design/`. Precisamos de **alta fidelidade visual** ao protótipo (`maestro-desktop-prototype.html`, `DESIGN-HANDOFF.md`) para entregar uma experiência coesa e profissional, sem regressão funcional.

Pré-requisito implementado: [maestro-ui-redesign](../maestro-ui-redesign/proposal.md) (shell, hub, editor tabs, i18n).

## What Changes

- **Design system OKLCH**: substituir tokens hex actuais por tokens extraídos do protótipo (background, surface, accent, radius, shadow, type scale, motion).
- **Shell visual**: header de janela com logo, toggle de tema e controlos estilo macOS; sidebar com ícones SVG e widget de sessão activa.
- **Navegação ampliada**: sidebar com **Sessões**, **Editor** (quando aplicável), **Captura** e **Definições** — Captura como rota de topo (não só tab do editor).
- **Hub redesenhado**: cards de perfil, toolbar (pesquisa, importar, criar), secções Fixadas/Todos, menu overflow — layout e estados do protótipo.
- **Editor redesenhado**: tabs Conteúdo/Captura/Avançado, form sections, lista de apps sequenciais, editor JSON — estilo do protótipo.
- **Ecrã Captura dedicado**: assistente de captura como vista principal acessível pela sidebar.
- **Activação estilo terminal**: overlay de timeline/logs conforme protótipo (substituir modal genérico actual).
- **Modais**: importação (e exportação onde existir) com estilo do protótipo.
- **Localização pt-PT**: manter português europeu (Guardar, Definições, etc.) apesar do protótipo estar em pt-BR.
- **Responsivo**: validar viewports do `DESIGN-MANIFEST.json` (360–1920px) sem scroll horizontal.

**Fora de scope (decisão fechada):**
- Mockup OS (topbar GNOME, system tray) — implementar só a janela Maestro.
- Integração real de system tray Tauri.

## Capabilities

### New Capabilities

- `design-system-handoff`: Tokens OKLCH, tipografia, sombras, motion e primitivos UI partilhados (btn, input, card, modal, tab) extraídos de `design/`.

### Modified Capabilities

- `application-shell`: Header de janela, toggle tema, nav com Captura, widget sessão activa na sidebar.
- `session-hub`: Layout visual hub-first com cards, toolbar e import modal conforme protótipo.
- `session-editor-layout`: Tabs e form sections com fidelidade visual ao protótipo.
- `assisted-profile-capture`: Entrada principal via rota **Captura** na sidebar; tab Captura no editor mantém captura contextual ou simplifica para link.
- `activation-results-ui`: Apresentação em overlay terminal conforme protótipo.
- `application-settings`: Form sections e controlos alinhados ao protótipo.
- `ui-localization`: Strings pt-PT para novos elementos (Captura, widget sessão activa, terminal overlay).
- `session-catalog-ux`: Cards, badges, estados hover/focus/disabled do protótipo.
- `activation-preview`: Dry-run integrado visualmente no fluxo de activação/terminal.

## Impact

- **Frontend**: `src/styles/tokens.css`, `themes.css`, `App.css`, `layout/`, `pages/`, `components/`, possivelmente novos `src/components/ui/` para primitivos.
- **Referência**: `design/maestro-desktop-prototype.html`, `design/DESIGN-MANIFEST.json`, `design/DESIGN-HANDOFF.md`.
- **Backend**: sem breaking changes; mesmos comandos Tauri existentes.
- **Docs**: actualizar `docs/smoke-test-checklist.md` com fluxo Captura sidebar e verificação visual.

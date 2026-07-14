## Context

React + Tauri 2. A UX redesign ([maestro-ui-redesign](../maestro-ui-redesign/design.md)) já entregou sidebar, hub, editor em tabs e i18n pt-PT. O visual actual usa tokens hex em [tokens.css](../../../src/styles/tokens.css) e CSS monolítico em [App.css](../../../src/App.css).

O design exportado em `design/` define o contrato visual:
- `maestro-desktop-prototype.html` — ecrã único com tokens OKLCH, layout e interacções
- `DESIGN-MANIFEST.json` — viewports, tokens obrigatórios, estados
- `DESIGN-HANDOFF.md` — sequência de implementação e fidelidade

**Decisões fechadas:** só janela Maestro (sem mockup OS/tray); Captura como rota sidebar; pt-PT; fidelidade pixel-perfect aos tokens e layout do protótipo.

## Goals / Non-Goals

**Goals:**
- Extrair e congelar tokens OKLCH do protótipo antes de refactor de componentes.
- Re-skin shell, hub, editor, captura, settings e overlay de activação conforme protótipo.
- Adicionar rota **Captura** na sidebar como ecrã dedicado.
- Manter toda a lógica Tauri existente (list, load, activate, capture, settings).
- Validar responsividade nos viewports do manifest (360–1920px).

**Non-Goals:**
- Mockup GNOME topbar, system tray ou integração tray real.
- Alterar schema JSON de perfis ou comandos Rust.
- Migrar para Tailwind ou nova lib de componentes.
- Mudar idioma para pt-BR.

## Decisions

| Decisão | Escolha | Alternativa descartada |
|---------|---------|------------------------|
| Fonte de tokens | OKLCH do protótipo em `:root` + `.light-mode` mapeados para `[data-theme]` | Manter hex actuais e “aproximar” cores |
| Estratégia CSS | Tokens em `tokens.css`; primitivos (`.btn`, `.card`, `.modal`) em `components.css`; layout por ecrã em `App.css` reduzido | CSS-in-JS ou Tailwind |
| Rotas | `hub \| editor \| capture \| settings` + `editorPath` | Manter capture só no editor |
| Editor na nav | Item **Editor** activo só quando `route === editor` | Esconder item Editor da sidebar |
| Tab Captura no editor | Manter tab com captura contextual (apps do perfil em edição); ecrã sidebar para captura global | Remover tab do editor |
| Theme toggle | Botão no header da janela; persiste via `ApplicationSettings.theme` existente | Toggle só em Definições |
| Sessão activa | Widget na sidebar footer com nome + **Desactivar** (quando backend expuser estado) ou última activação | Sem widget |
| Resultados activação | Overlay estilo terminal do protótipo (logs + timeline) | Manter modal genérico |
| Import | Modal dedicado (protótipo) em vez de `<input type="file">` oculto | Manter import nativo |
| Ícones | SVG inline (copiar paths do protótipo) | Icon library externa |
| Fidelidade | Screenshot diff manual nos viewports do manifest antes de merge | Sem verificação visual |

## File layout

```
design/                          # read-only reference (não alterar)
src/styles/
  tokens.css                     # OKLCH tokens from prototype
  themes.css                     # data-theme ↔ light-mode bridge
  components.css                 # btn, input, card, modal, tabs, terminal
src/layout/
  AppShell.tsx                   # window header + theme toggle
  Sidebar.tsx                    # 4 nav items + session widget
src/pages/
  SessionHubPage.tsx             # reskin + import modal
  CaptureAssistantPage.tsx       # NEW standalone capture
src/components/
  ProfileEditorScreen.tsx        # reskin tabs/forms
  SettingsScreen.tsx             # reskin sections
  ActivationTerminalOverlay.tsx  # NEW terminal-style results
  ImportProfileModal.tsx         # NEW
src/i18n/pt.ts                   # new strings (Captura, Desactivar, etc.)
```

## Token mapping

Extrair do protótipo (`:root` / `.light-mode`):

| Protótipo | App (`data-theme`) |
|-----------|-------------------|
| `--bg` | `--color-bg` |
| `--surface` | `--color-surface` |
| `--surface-hover` | `--color-surface-hover` |
| `--fg` | `--color-text` |
| `--muted` | `--color-text-muted` |
| `--border` | `--color-border` |
| `--accent` | `--color-accent` |
| `--success/warning/danger` | `--color-success/warning/danger` |
| `--radius-*`, `--shadow-*` | manter nomes |
| `--font-display/body/mono` | `--font-sans`, `--font-mono` |

## Migration Plan

1. **Fase A — Tokens & primitivos:** substituir `tokens.css`, adicionar `components.css`, actualizar `themes.css`.
2. **Fase B — Shell:** header, sidebar nav (4 items), widget sessão, theme toggle.
3. **Fase C — Hub:** cards, toolbar, import modal, overflow menu styling.
4. **Fase D — Editor & Settings:** tabs, form sections, grouped settings.
5. **Fase E — Captura:** nova página + wiring em `App.tsx`.
6. **Fase F — Terminal overlay:** substituir modal de resultados.
7. **Fase G — QA visual:** viewports manifest + smoke checklist.

Rollback: tokens antigos ficam no git; revert por fase se regressão funcional.

## Risks / Trade-offs

| Risco | Mitigação |
|-------|-----------|
| Regressão CSS grande | Fases incrementais; manter classes funcionais onde possível |
| Duplicação Captura (sidebar + tab) | Tab editor = contextual; sidebar = descoberta global |
| Widget “sessão activa” sem API backend | MVP: mostrar última sessão activada em memória/localStorage |
| Protótipo pt-BR | Mapa explícito pt-BR→pt-PT em `pt.ts` |
| OKLCH em browsers antigos | Tauri WebKit moderno suporta OKLCH; fallback = mesmos valores em sRGB se necessário |

## Open Questions

- **Desactivar sessão:** o backend expõe comando de teardown? Se não, widget mostra estado informativo até existir API.
- **Export modal:** protótipo inclui export modal — replicar se já existir export via overflow, ou adiar para polish.

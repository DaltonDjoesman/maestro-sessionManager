## Context

Maestro já expõe `list_assistant_running_apps` em `capture/assistant.rs`: enumera processos com `sysinfo`, aplica `denylisted_basename` + `assistant_background_noise`, deduplica por executável (editores por `cwd_hint`), e devolve até 200 candidatos. A UI em `ProfileEditorScreen` mostra cards com PID, executable e cmdline — mental model de task manager, não de seleção de apps para um perfil de sessão.

O design original (`maestro-mvp`) exclui **controlar** workspaces de janelas. Esta change adiciona apenas **leitura** opcional para agrupar o assistente — alinhado com o propósito de captura assistida.

## Goals / Non-Goals

**Goals:**

- Lista predefinida mostra só candidatos classificados como **app** (GUI / `.desktop` conhecido).
- Nomes e ícones legíveis; ruído típico Pop!_OS/Cosmic filtrado ou rebaixado a “processo”.
- Agrupar cards por **workspace desktop** quando o adaptador conseguir resolver (fase 2).
- Manter `cwd_hint` para editores; não quebrar “Add selected to draft”.
- Testes unitários para novos filtros e classificação (TDD conforme `backend-dev-workflow`).

**Non-Goals:**

- Mover ou focar janelas entre workspaces.
- Garantir workspace em Wayland puro sem API do compositor.
- Substituir edição manual de executável/args no perfil.
- Ícones rasterizados no bundle (fase 1: nome Freedesktop ou placeholder melhorado; fase 2 opcional: resolver path PNG via tema GTK).

## Decisions

| Decisão | Escolha | Alternativas | Racional |
|---------|---------|--------------|----------|
| App vs processo | **Híbrido**: match `.desktop` `Exec`/`Icon` → `app`; denylist/noise → omitido; resto → `process` (oculto por defeito) | Só allowlist estrita (esconde AppImages sem .desktop) | Balanceia precisão e cobertura; utilizador avançado vê processos com toggle |
| Cache `.desktop` | Indexar em memória no primeiro scan da sessão (`HashMap<basename, DesktopEntry>`) | Re-parse por PID | Centenas de ficheiros em `/usr/share/applications`; scan único por refresh |
| Match executável ↔ desktop | Normalizar: basename, `%u/%U/%f` stripped, comparação com `proc.exe()` e argv0 | Só basename | Flatpak/bwrap precisam de heurística; basename cobre maioria |
| Filtros novos | Centralizar em `assistant_background_noise` + entradas em `denylisted_basename` | Filtro só na UI | Uma fonte de verdade; testes já existentes em `assistant.rs` |
| Ruído `node`+`npm` | Filtrar quando cmd contém `npm run`, `npx`, ou `node` sem janela associada (fase 2); fase 1: filtrar `npm run`/`npx` sempre | Filtrar todo `node` | Dev servers são processos; utilizador pode precisar de terminal — toggle processos |
| `oosplash` / LibreOffice | Mapear para app pai via `.desktop` (`libreoffice-calc` etc.) ou filtrar splash se `soffice` principal existir | Mostrar oosplash | Splash confunde; preferir janela `soffice.bin` quando existir |
| Workspace X11 | Módulo `platform/workspace_x11.rs`: `x11rb` ou invocação controlada de `wmctrl -l -p` atrás de feature flag | Só documentar | `wmctrl` evita dependência pesada no MVP da fase 2; avaliar `x11rb` se quisermos zero subprocess |
| Workspace Wayland/Cosmic | Spike: `busctl` / cosmic shell D-Bus; se falhar, `desktopWorkspace: null` | Bloquear fase 2 | Best-effort; não bloquear fase 1 |
| DTO | Campos opcionais serde: `kind`, `displayName`, `iconName`, `desktopWorkspace` | Novo comando separado | Um refresh; UI simples |
| UI agrupamento | Ordenar: workspace (numérico) → `displayName` A–Z; secção “Outros processos” colapsada | Flat list | Espelha Cosmic overview mental model |

## Risks / Trade-offs

| Risco | Mitigação |
|--------|-----------|
| AppImage sem `.desktop` classificado como processo | Toggle “Mostrar processos”; documentar; heurística futura por `StartupWMClass` |
| `wmctrl` ausente no sistema | Detetar no spike; workspace null; sem erro fatal |
| Falsos negativos (app real filtrada) | Toggle processos; testes com fixtures reais do utilizador |
| Performance do scan `.desktop` | Cache por refresh; limite 200 candidatos mantido |
| Divergência X11 vs Wayland no mesmo código | `platform_name` + `session_type` no início do comando |

## Migration Plan

1. **Fase 1**: estender DTO + filtros + classificação + UI (sem dependência de workspace).
2. **Fase 2**: adaptador workspace; agrupamento UI; documentar limitações Wayland em `docs/linux-desktop.md`.
3. Rollback: campos novos são opcionais; UI antiga ignora campos desconhecidos.

## Open Questions

- Fase 2: preferir subprocess `wmctrl` vs crate `x11rb` puro Rust?
- Resolver ícone para path PNG via `gtk-icon-theme` ou manter só `iconName` string na UI?
- Limite máximo de workspaces a mostrar (ex.: 1–16 no Cosmic)?

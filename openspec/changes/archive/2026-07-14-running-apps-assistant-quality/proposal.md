## Why

O assistente de captura no editor de perfis enumera **processos** via `sysinfo` com filtros heurísticos, mas a UI apresenta-os como “apps”. Daemons (`obexd`), runtimes de desenvolvimento (`node` + `npm run …`) e processos de arranque (`oosplash`) passam o filtro e confundem o utilizador. Falta também contexto de **workspace do desktop** (Cosmic/GNOME) para agrupar o que está aberto em cada espaço de trabalho — informação útil ao montar perfis de sessão sem violar o non-goal de controlar janelas.

## What Changes

### Fase 1 — Classificação e UI (entrega imediata)

- **Classificação app vs processo**: candidatos marcados como `kind: "app" | "process"`; por defeito a lista mostra só apps; toggle “Mostrar processos” para avançado.
- **Filtros alargados**: ruído adicional (`obexd`, `oosplash`/`soffice` splash, `node`+`npm`/`npx`, runtimes órfãos); reforço da denylist partilhada com `platform/linux.rs`.
- **Labels humanos**: resolver nome e ícone via entradas **Freedesktop** (`.desktop` → `Name`, `Icon`, `Exec`); fallback para basename do executável.
- **UI do assistente**: renomear secção para “Apps em execução”; cards com ícone real, nome legível, menos destaque a PID/cmdline; agrupar por **nome de app** quando múltiplas instâncias (editores já separados por `cwd_hint`).
- **DTO estendido**: `RunningAppCandidate` ganha `kind`, `displayName`, `iconName` (opcional), `desktopWorkspace` (opcional, fase 2).

### Fase 2 — Workspace do desktop (spike, best-effort)

- Detetar `XDG_SESSION_TYPE` (X11 vs Wayland).
- **X11**: mapear PID → janela → `_NET_WM_DESKTOP` (EWMH) via adaptador Linux.
- **Wayland / Cosmic**: spike D-Bus ou documentar limitação; UI agrupa por workspace quando disponível, senão secção única “Sem workspace”.
- **Sem mover janelas** — apenas leitura para o assistente.

## Capabilities

### New Capabilities

- `running-apps-classification`: Detecção Freedesktop, filtros alargados, `kind` app/process, labels e ícones.
- `desktop-workspace-hints`: Leitura best-effort do workspace virtual por janela/PID no Linux; agrupamento na UI do assistente.

### Modified Capabilities

- `assisted-profile-capture`: Requisitos da lista de candidatos, apresentação UI (agrupamento, toggle processos, copy/labels), e contrato do comando Tauri `list_assistant_running_apps`.

## Impact

- **Backend**: `src-tauri/src/capture/assistant.rs`, novo módulo `capture/desktop.rs` ou extensão de `platform/linux.rs`; possível crate `freedesktop_entry` ou parse manual de `.desktop`; testes unitários nos filtros e classificação.
- **Frontend**: `ProfileEditorScreen.tsx`, `src/types/capture.ts`, CSS dos cards; agrupamento por workspace e por `displayName`.
- **API Tauri**: resposta de `list_assistant_running_apps` com campos novos (compatível: campos opcionais com defaults).
- **Risco**: Wayland/Cosmic pode não expor workspace — fase 2 degrada graciosamente; classificação `.desktop` pode falhar em AppImages sem entrada instalada (fallback para processo ou heurística).

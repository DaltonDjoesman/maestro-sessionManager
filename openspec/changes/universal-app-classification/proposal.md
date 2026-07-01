## Why

A classificação **app vs processo** no assistente de captura ainda depende de uma **lista hardcoded** (`is_known_gui_basename`) quando o match Freedesktop falha — frágil entre PCs, instalações Flatpak/Snap/AppImage e paths custom. O agrupamento por workspace também sofre porque a lista nasce de **processos** (`sysinfo`) em vez de **janelas visíveis**. Precisamos de um classificador **portável** que funcione independentemente de como a app foi instalada.

## What Changes

### Fase A — Classificador universal (backend)

- **Índice `.desktop` alargado**: Flatpak exports, Snap desktop files, além de `/usr/share/applications` e `~/.local/share/applications`.
- **Parse de `Exec` robusto**: extrair binário real após `bwrap`, `flatpak-spawn`, AppImage wrappers; indexar `TryExec` e `StartupWMClass`.
- **Classificação por score**: janela visível (+ forte), match `.desktop` (+ forte), sessão gráfica do utilizador; denylist de ruído (negativo). **Remover** dependência da allowlist hardcoded como caminho principal.
- **Regra app**: `kind: app` quando score ≥ limiar (janela OU `.desktop` forte); sem janela e sem desktop → `process`.
- **Rebaixar tray/background**: app com `.desktop` mas sem janela mapeada → `process` (oculto por defeito).

### Fase B — Descoberta ancorada em janelas

- Pipeline **window-first** no X11: `wmctrl -l -p` (e futuro compositor) como fonte primária de candidatos; enriquecer com processo + `.desktop`.
- **Um card por janela** quando o mesmo PID tem janelas em workspaces diferentes (ex.: Vivaldi, multi-tab browser).
- Deduplicação por `(executable, desktop_workspace, window_title_key)` em vez de só executável.
- Campos opcionais no DTO: `windowTitle`, `classificationConfidence` (`high` | `medium` | `low`).

### Fase C — Documentação e remoção de dívida

- Remover ou reduzir `is_known_gui_basename` a zero entradas (só testes/fixtures se necessário).
- Atualizar `docs/linux-desktop.md` e smoke checklist com matriz install-type × sinal de classificação.

## Capabilities

### New Capabilities

- `app-classifier-scoring`: Modelo de score, limiares, testes unitários por cenário (deb, flatpak, appimage, daemon).
- `window-anchored-discovery`: Candidatos derivados de janelas mapeadas; enriquecimento processo/desktop; multi-row por workspace.

### Modified Capabilities

- `running-apps-classification`: Índice Freedesktop alargado, parse Exec/StartupWMClass; **REMOVED** requisito de allowlist hardcoded como classificador primário.
- `desktop-workspace-hints`: Workspace por **janela** (não só PID pós-dedupe de processo).
- `assisted-profile-capture`: Contrato da lista (multi-janela, confiança, toggle processos inalterado na UX).

## Impact

- **Backend**: `capture/desktop_index.rs`, novo `capture/classifier.rs`, refactor `capture/assistant.rs`, `platform/workspace.rs`.
- **Frontend**: agrupamento pode mostrar subtítulo com título de janela quando multi-row; badge opcional de confiança baixa.
- **API**: campos novos opcionais em `RunningAppCandidate`; compatível com UI actual.
- **Risco**: Wayland sem wmctrl — fase B degrada para process-first + `.desktop` até API Cosmic; score evita falsos positivos.

## Context

O change `running-apps-assistant-quality` entregou `kind`, `displayName`, workspace via `wmctrl`, e índice `.desktop` básico. Em produção no Pop!_OS/Cosmic (X11 híbrido) restam:

- Allowlist hardcoded em `is_known_gui_basename` (ticktick, eog, …).
- Candidatos listados por **processo** → dedupe por executável perde workspace e omite apps.
- Match `.desktop` falha em Flatpak (`bwrap`, `/app/…`) e Snap.
- Browsers Electron: um PID, várias janelas/workspaces → um card só.

## Goals / Non-Goals

**Goals:**

- Classificar **app vs processo** sem lista branca de nomes de binários.
- Funcionar para deb, Flatpak, Snap, AppImage com janela, e installs em `/opt/` ou `~/Applications/`.
- Workspace e agrupamento corretos via **janela** no X11.
- Testes unitários por fixture (Exec parse, score, window merge).
- Manter toggle “Show processes” e fluxo “Add selected to draft”.

**Non-Goals:**

- Controlar ou mover janelas.
- Garantir paridade Wayland no MVP deste change (documentar + degradar).
- Resolver ícones PNG via GTK theme (continuar `iconName` string).
- Substituir edição manual de executável no perfil.

## Decisions

| Decisão | Escolha | Alternativas | Racional |
|---------|---------|--------------|----------|
| Fonte primária (X11) | **Janelas** (`wmctrl`) → processo → `.desktop` | Process-first (actual) | Janela = app aberta pelo utilizador; install-agnostic |
| Fonte primária (sem wmctrl) | Process + `.desktop` + score | Só `.desktop` | Wayland fallback até compositor API |
| Classificação | **Score integer** com limiar 80 = app | Árvore if/else | Extensível; testável por tabela |
| Allowlist hardcoded | **Remover** após fase A | Manter fallback | Dívida técnica; score + janela substituem |
| `.desktop` paths | Standard + Flatpak exports + Snap | Só `/usr/share` | Cobre installs normais em qualquer PC |
| Exec parse | Regex/heurística para último token útil após wrappers | Só primeiro token | Flatpak `bwrap … /app/foo` |
| StartupWMClass | Indexar; match wmctrl title/class quando existir | Ignorar | Apps com Exec wrapper |
| Multi-janela | **1 candidato por janela** se desktops ou títulos diferem | 1 por PID | Vivaldi/TickTick multi-workspace |
| Dedupe key | `(stable_app_key, desktop, title_hash)` | executable only | Preserva workspace rows |
| DTO novos | `windowTitle?`, `classificationConfidence?` | Reuso só displayName | UI pode mostrar “WhatsApp - Vivaldi” |
| Tray sem janela | Score `.desktop` but −50 sem janela → process | Esconder totalmente | OBS background, nautilus service edge cases |

## Score model (normative for implementation)

| Signal | Points |
|--------|--------|
| Window mapped (pid or ancestor) | +100 |
| `.desktop` Exec/TryExec match | +80 |
| StartupWMClass match | +60 |
| Flatpak/Snap path hint (`/app/`, `/snap/`) | +20 |
| User session (has DISPLAY / XDG_SESSION_TYPE) | +10 |
| Denylist / background_noise hit | −1000 (exclude) |
| `.desktop` match but no window | −50 |

- **app** if score ≥ 80 and not excluded.
- **process** otherwise.
- **confidence**: high (window + desktop), medium (window OR desktop), low (neither, only for toggle).

## Architecture sketch

```
wmctrl / compositor          sysinfo (fallback)
        │                           │
        └──────────┬────────────────┘
                   ▼
         WindowAnchoredDiscovery
                   │
                   ▼
         DesktopIndex (expanded)
                   │
                   ▼
            AppClassifier::score
                   │
                   ▼
         RunningAppCandidate[]
```

## Risks / Trade-offs

| Risco | Mitigação |
|--------|-----------|
| wmctrl ausente | Fallback process-first; doc |
| Performance scan `.desktop` | Cache por refresh; ~500–2000 files OK |
| Duplicados multi-janela | Dedupe key estável; UI agrupa por workspace |
| Falsos apps (terminal com janela) | Denylist + requer `.desktop` OU title heurística para shells |
| Breaking UI sort order | Campos opcionais; sort by displayName + windowTitle |

## Migration Plan

1. Ship expanded `DesktopIndex` + `AppClassifier` (process list unchanged behaviour initially).
2. Switch pipeline to window-first behind `session_type == x11` check.
3. Remove `is_known_gui_basename` usages; delete constant.
4. Add multi-window rows; adjust frontend labels.
5. Update smoke checklist.

## Open Questions

- Expor score numérico na API ou só `classificationConfidence` enum?
- Filtrar terminais (`kitty`, `alacritty`) como app sempre que têm janela — desejável para perfis dev?
- Limite máximo de janelas listadas (ex.: 100)?

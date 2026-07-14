## Context

Maestro (Tauri + React + Rust) já tem catálogo de perfis, editor, activação com `ActivationStepSummary`, e `ApplicationSettings.theme` persistido. A shell da app (`app-shell`) ainda não reage ao tema; não há estado de “última sessão” nem pins; activação mostra tabela simples; não existe comando de pré-visualização; export/import são manuais via filesystem.

## Goals / Non-Goals

**Goals:**

- Hub de sessões com descoberta rápida (pesquisa, badges, continuar última, pins).
- Resumo de activação escaneável e acção “ver log”.
- Dry-run que devolve a mesma forma de passos ou DTO dedicado **sem** `exec` real.
- `theme` → `data-theme` / classes na raiz + tokens CSS light/dark.
- Export/import/duplicate-with-name com validação de `schema_version` e paths seguros.
- Documentar fase 2 `.desktop` (empacotamento, `Exec`, MIME).

**Non-Goals:**

- Reintroduzir **context-cleanup** ou matar processos por divergência.
- Controlar geometria de janelas / Wayland window placement.
- Cloud sync ou contas.

## Decisions

| Decisão | Escolha | Alternativas | Racional |
|----------|---------|--------------|-----------|
| Onde guardar pins / última sessão | `localStorage` + chave por `profiles_root` hash curto | Estender `settings.json` com `ui_state` | Evita migrar schema de settings; reset natural se mudar pasta de perfis |
| Dry-run | Novo comando `preview_session_activation(profile)` devolvendo lista de `{ kind, label, argv, cwd? }` sem spawn | Reutilizar `activate` com flag interna | API explícita; menos risco de branch esquecida em `activate` |
| “Abrir log” | Fase 1: `tauri-plugin-opener` em path de ficheiro de log sob data dir; se ainda não existir log, abrir Settings ou toast | Só in-app viewer | Entrega valor rápido; viewer pode ser ADDED depois |
| Import perfil | Dialog escolhe ficheiro → validação → `save_session_profile` com nome novo ou overwrite confirmado | Drag-drop | Tauri dialog é MVP simples |
| `.desktop` fase 2 | Documentar em `linux-desktop-entry` spec; implementação após bundle `identifier` estável | Bloquear esta change até empacotar | Desacopla UX da pipeline de release |

## Risks / Trade-offs

| Risco | Mitigação |
|--------|-----------|
| Import JSON malicioso / path traversal | Validar só JSON conhecido; normalizar nome de ficheiro; nunca executar conteúdo importado |
| Dry-run diverge do activate real | Partilhar função de “build steps” usada por ambos; testes de paridade |
| localStorage cheio / privado em máquina partilhada | Pins opcionais; documentar; não guardar segredos |
| Tema light com contraste fraco | Checklist visual + tokens mínimos WCAG-ish para ambos |

## Migration Plan

1. Ship UI + storage de pins/última sessão (sem quebrar perfis existentes).
2. Ship preview command + UI; depois timeline de resultados.
3. Ship export/import + duplicate-with-name.
4. Tema: uma PR focada em CSS + root attribute.
5. `.desktop`: após `tauri build` validado em alvo deb.

## Open Questions

- Log único por sessão vs por app: rotação e tamanho máximo?
- Pins com limite máximo (ex.: 12)?
- Duplicate-with-name: restringe caracteres do nome de ficheiro a `[a-z0-9-_]+`?

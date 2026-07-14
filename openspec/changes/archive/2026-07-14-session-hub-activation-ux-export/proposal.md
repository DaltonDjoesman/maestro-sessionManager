## Why

O Maestro já cobre CRUD de perfis e activação, mas a **home/sessões** ainda é um catálogo simples, a **activação** devolve passos difíceis de escanear, não há **pré-visualização** dos comandos antes de executar, o **tema** nas definições não se reflecte na shell da app, e falta **portabilidade** (exportar/importar) e **rituais** (última sessão, pins). Integrações Linux (`.desktop`) e polish visual fecham o ciclo de confiança e adopção.

## What Changes

- **Home / catálogo de sessões**: “Continuar última sessão”, **pins** (ordem ou favoritos), **pesquisa por nome/ficheiro**, **badges** (ex.: só browser, N aplicações, inválido).
- **Pós-activação**: resumo mais legível — **timeline** ou cartões por passo, **ícones** ok / aviso / falha, link **“Abrir log”** (destino a definir em design: ficheiro vs painel in-app).
- **Dry run / pré-visualização**: modo que mostra **argv** (e eventualmente `cwd`) que seriam usados **sem** lançar processos; opcionalmente validações só leitura.
- **Tema (Settings)**: ligar `theme` persistido ao **CSS / data-theme** da app (system/light/dark) e polish de contrastes para ambos.
- **Linux (fase 2)**: ficheiro **`.desktop`** e fluxo “Abrir com Maestro” (MIME / `%f`); pode ficar atrás de feature flag ou milestone separado.
- **Dados**: **exportar** perfil JSON, **importar** (merge ou novo ficheiro), **duplicar com nome** (além do duplicar actual que gera nome automático).

**Nota:** A funcionalidade **context-cleanup** foi removida do produto; esta change **não** a reintroduz. Os specs históricos em `maestro-mvp` podem ser marcados como obsoletos ou substituídos por deltas na fase de specs.

## Capabilities

### New Capabilities

- `session-catalog-ux`: Hub de sessões na home — última sessão, pins, pesquisa, badges de conteúdo.
- `activation-results-ui`: Apresentação rica do resumo de activação (timeline, ícones, log).
- `activation-preview`: Dry-run / pré-visualização de argv antes do spawn.
- `profile-portability`: Exportar, importar, duplicar com nome explícito.
- `linux-desktop-entry`: Integração `.desktop` / “Abrir com Maestro” (fase 2, Linux).

### Modified Capabilities

- `application-settings`: O valor `theme` SHALL afectar a shell da aplicação (system/light/dark), não apenas persistência.
- `session-activation`: SHALL suportar um modo de pré-visualização (dry-run) que não executa spawns; requisitos de resumo podem alinhar-se com `activation-results-ui`.

## Impact

- **Frontend**: `App.tsx`, catálogo, editor, fluxo de activação, CSS global, possível estado global (última sessão, pins) — **localStorage** ou settings estendidos (design).
- **Backend**: novos comandos Tauri para dry-run, export/import; possível log estruturado ou path exposto para “abrir log”.
- **Empacotamento Linux**: fase 2 pode tocar em `tauri.conf.json`, ficheiros em `/usr/share/applications`.
- **Risco / BREAKING**: importar perfil malicioso (JSON) — validação estrita e sandbox de paths; dry-run não deve escrever ficheiros de perfil sem confirmação.

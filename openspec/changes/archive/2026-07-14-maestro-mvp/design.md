
## Context

O repositório é um tutorial OpenSpec sem código do Maestro ainda. A proposta `maestro-mvp` define um desktop app **Tauri + Rust** para **Linux** que guarda **perfis de sessão** em JSON, **activa** sessões (lançar IDE/apps e browser com URLs), e opcionalmente sugere **encerramento gracioso** de processos divergentes. O utilizador-alvo inicial usa **Pop!_OS**; outros SO ficam para adaptadores futuros.

## Goals / Non-Goals

**Goals:**

- Entregar um **MVP instalável** no Linux que cumpre os critérios de sucesso da proposta (perfis JSON, activação, browser com lista de URLs numa nova janela com isolamento configurável, limpeza opcional com confirmação).
- Separar **motor Rust** (ficheiros, processos, construção de comandos) da **UI Tauri** (comandos IPC bem definidos, erros serializáveis).
- **Adaptador `PlatformContext`** (trait ou módulo) para listagem de processos e sinais no Linux, deixando hooks vazios ou stubs para macOS/Windows.
- **Schema versionado** de perfis e de configuração global para migrações futuras.

**Non-Goals:**

- Controlar geometria, z-order ou workspace virtual de janelas.
- Sincronização na nuvem ou contas.
- Garantir `file://` em todas as sandboxes de browser.
- Detecção automática 100% fiável do “projecto aberto” sem confirmação humana.
- Paridade de empacotamento macOS/Windows no MVP.

## Decisions

| Decisão | Escolha | Racional | Alternativas consideradas |
|--------|---------|----------|---------------------------|
| Shell do projecto | **Tauri 2** + Rust 2021 | UI desktop nativa, empacotamento razoável, IPC com o core. | **Electron** (maior footprint); **só CLI** (pior UX para não-técnicos). |
| Layout do repo | Raiz `src-tauri/` + `src/` frontend (React/Vue/Svelte a escolher na implementação) | Convenção Tauri standard. | Monorepo Cargo workspace + UI separada (mais complexo para MVP). |
| Listagem de processos Linux | Crate **`sysinfo`** (ou equivalente maduro) atrás do adaptador | Menos código inseguro que parse manual de `/proc` no MVP. | Leitura directa de `/proc` (mais controlo, mais bugs). |
| Arranque de subprocessos | `tokio::process` ou `std::process::Command` conforme necessidade de async | Tauri já async-friendly. | `daemonize` (não necessário). |
| Isolamento do browser | **Chromium:** `--user-data-dir` por sessão ou workspace; **Firefox:** perfil dedicado + `-no-remote` quando necessário | Garante nova instância/janela sem colidir com o browser “pessoal”. | Só `--new-window` sem perfil (reutilização imprevisível). |
| Kill switch | **SIGTERM** por omissão; **SIGKILL** só com opt-in explícito na UI e timeout | Alinha com encerramento gracioso e expectativas do utilizador. | SIGKILL imediato (dados perdidos). |

## Risks / Trade-offs

| Risco | Mitigação |
|--------|-----------|
| Flatpak/snap alteram argv e visibilidade de ficheiros | Documentar; permitir path absoluto ao executável “host”; avisos na UI para `file://`. |
| Falsos positivos no diff por nome de processo | Lista de exclusão por omissão + correspondência configurável; nunca listar PID 1/init. |
| Utilizador edita JSON manualmente e quebra o schema | Validação na leitura + mensagens de erro com campo; `schema_version` para migradores. |
| Tamanho do bundle Tauri + WebView | Aceitável para desktop; não é alvo mobile. |

## Migration Plan

- **Deploy:** primeira versão como artefacto de build Tauri para Linux (formato a escolher na implementação: deb, AppImage, ou binário + instruções).
- **Rollback:** utilizador mantém cópias dos JSON de perfis; desinstalar a app não remove a pasta de dados se documentado em contrário; preferir **não apagar** perfis ao desinstalar.
- **Evolução de schema:** ao subir `schema_version`, o motor aplica migrador registado ou recusa carregar com mensagem clara até o utilizador guardar de novo pela UI.

## Open Questions

- Qual stack de frontend (React, Vue, Svelte, Leptos-in-Tauri) para a equipa — deixar aberto na primeira tarefa de scaffolding.
- Formato de distribuição primário (deb vs AppImage) para CI e testes em Pop!_OS.
- Se a **captura assistida** entra no mesmo marco que o MVP instalável ou milestone seguinte (a proposta admite MVP+).

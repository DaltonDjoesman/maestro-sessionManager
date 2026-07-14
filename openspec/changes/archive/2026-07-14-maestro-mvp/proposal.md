## Why

Alternar entre modos de trabalho (desenvolvimento, estudo, lazer) custa tempo e atenção: abrir o IDE no repositório certo, o browser com a documentação certa, ferramentas auxiliares, e reduzir distracções. O **Maestro** resolve isto orquestrando **processos e dados locais** (sem controlar geometria de janelas, evitando atrito com Wayland), com uma app desktop **Tauri + Rust** e perfis em **JSON** versionáveis.

## What Changes

- Introduz o produto **Maestro (MVP)**: gestor de sessões de ambiente de trabalho para **Linux** (referência Pop!_OS), com camada de **adaptadores** preparada para outros SO.
- **Perfis de sessão** em JSON (`schema_version`, metadados, comandos de aplicação, bloco de navegador, regras opcionais).
- **Lançador**: executável + args + `cwd` opcional; activação sequencial com intervalos opcionais; erros reportados à UI.
- **Navegador**: nova janela com lista de URLs (tabs); regras **Chromium** (`--new-window`, `--user-data-dir` recomendado) e **Firefox** (`-new-window`, perfil dedicado, `-no-remote` quando necessário); `https://` como fluxo principal; `file://` com avisos onde sandbox/políticas falhem.
- **Activar sessão**: fluxo único que valida perfil, resolve caminhos, executa plano e devolve resumo (sucesso / falha / avisos); política “não lançar se já a correr” opcional no MVP.
- **Limpeza suave**: comparação de processos relevantes vs sessão; lista de exclusão; encerramento só após **confirmação**; **SIGTERM** primeiro; **SIGKILL** só como último recurso e explícito.
- **Definições globais**: pasta de perfis, defaults de browser, políticas diff/kill, tema, logs.
- **Captura assistida (MVP+)**: assistente para sugerir apps e `cwd` de editores no Linux; sempre editável pelo utilizador — pode ficar para fase posterior à primeira entrega instalável.

Não há alterações a capabilities existentes no repositório (**nenhum spec principal prévio** neste tutorial).

## Capabilities

### New Capabilities

- `session-profiles`: Modelo de dados, `schema_version`, CRUD de perfis, ficheiros JSON no disco, duplicação e edição (UI + ficheiro).
- `process-launcher`: Arranque de aplicações e IDEs na activação (`Command` com executável, args, `cwd`), sequência e intervalos, erros claros.
- `browser-launch`: Construção da linha de comandos por família (Chromium vs Firefox), nova janela com múltiplos URLs, isolamento via `user-data-dir` / perfil dedicado.
- `session-activation`: Fluxo “Activar sessão”, validação, resumo para UI, políticas opcionais (ex.: skip se já a correr).
- `context-cleanup`: Listagem de processos (Linux), diff vs perfil, UI de confirmação, SIGTERM/SIGKILL com salvaguardas.
- `application-settings`: Configuração global persistida, lida ao arranque, editável na UI.
- `assisted-profile-capture`: Assistente de snapshot (lista de candidatos, sugestão de `cwd` para editores conhecidos); requisitos podem marcar-se como pós-MVP mínimo instalável.

### Modified Capabilities

- *(Nenhum — não existem specs em `openspec/specs/` neste repositório.)*

## Impact

- **Novo** código: projecto **Tauri + Rust** (ou monorepo a definir em `design.md`), comandos Tauri entre UI e motor.
- **Dependências**: crates para processos e listagem de processos no Linux (ex.: `sysinfo` ou equivalente), serialização JSON (`serde`).
- **Sistemas**: apenas **Linux** no MVP; empacotamento (deb/AppImage/etc.) alvo Pop!_OS; sem nuvem obrigatória.
- **Risco**: variabilidade de instaladores de browser (Flatpak), `file://`, e nomes de executáveis — mitigado por configuração explícita e documentação.

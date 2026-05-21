## Why

O núcleo Rust do Maestro (Tauri `src-tauri`) beneficia de um fluxo explícito de testes e verificação; sem convenções documentadas, contribuições podem saltar TDD ou commits grandes, aumentando risco de regressões e dificultando revisão.

## What Changes

- Documenta e formaliza que trabalho **orientado ao backend** (Rust em `src-tauri/`, comandos Tauri, integração com disco/processos) **deve** seguir as skills **Superpowers** relevantes (em especial test-driven-development e verification-before-completion), com referência aos caminhos no repositório.
- Define requisitos leves em spec para **commits pequenos e descritivos** ao longo da implementação de cada change OpenSpec aplicável ao backend.
- Não altera comportamento da app em runtime; é **processo e documentação** para agentes e humanos.

## Capabilities

### New Capabilities

- `backend-dev-workflow`: Convenções para desenvolvimento do backend Maestro: uso obrigatório das skills Superpowers indicadas, estrutura de testes Rust, e política de commits.

### Modified Capabilities

- *(Nenhum — não existem specs em `openspec/specs/` para alterar.)*

## Impact

- Novos ficheiros em `openspec/changes/backend-superpowers-workflow/` (proposal, design, specs, tasks).
- Opcionalmente `README.md` ou `.cursor/rules` podem referenciar esta change após `/opsx:apply`; fora do âmbito mínimo desta proposta.

## Context

O repositório já inclui skills Superpowers em `.agents/skills/` e no plugin cache (TDD, verification-before-completion, systematic-debugging, etc.). O backend Maestro vive em `src-tauri/` (Rust 2021, Tauri 2). A proposta `backend-superpowers-workflow` formaliza como agentes e contribuidores devem usar essas skills ao tocar em código Rust/comandos Tauri.

## Goals / Non-Goals

**Goals:**

- Tornar explícito **qual** skill aplicar antes de implementar ou corrigir backend.
- Exigir **testes Rust** (`cargo test`) para lógica nova ou alterada em `src-tauri/src/**` quando aplicável.
- Exigir **commits pequenos** com mensagens descritivas por unidade lógica (task OpenSpec, sub-tarefa, ou fix isolado).

**Non-Goals:**

- Alterar CI obrigatório ou hooks de git (a menos que outra change o faça).
- Prescrever stack frontend (React) além do necessário para comandos Tauri.
- Substituir o fluxo OpenSpec existente (`maestro-mvp`); esta change é **complementar**.

## Decisions

| Decisão | Escolha | Racional |
|--------|---------|----------|
| Skills obrigatórias (backend) | **test-driven-development** antes de código de produção; **verification-before-completion** antes de afirmar “passa” ou pedir merge | Red regressões e afirmações sem prova |
| Skills recomendadas | **systematic-debugging** em bugs/falhas de teste; **using-superpowers** no início de tarefas para não saltar o pipeline | Consistência com o ecossistema Cursor do repo |
| Onde documentar paths | Spec `backend-dev-workflow` + `design.md`; opcionalmente AGENTS.md numa change futura | Spec é a fonte normativa para `/opsx:apply` |
| Commits | Um commit por **sub-tarefa** concluída (checkbox em `tasks.md`) ou por **fix mínimo** com mensagem `type(scope): descrição` | Revisão e bisect mais fáceis |

## Risks / Trade-offs

| Risco | Mitigação |
|--------|-----------|
| Fricção para contribuidores externos | Spec é curta; exceções por PR com justificação |
| TDD “demais” para boilerplate trivial | Tasks podem marcar “sem teste novo” apenas quando não há comportamento novo |

## Migration Plan

- Após merge/archive desta change, referenciar a spec em onboarding ou `README` se desejado.
- Nenhuma migração de dados.

## Open Questions

- Se `openspec/specs/` global existir no futuro, considerar mover requisitos de processo para lá (fora de `changes/`).

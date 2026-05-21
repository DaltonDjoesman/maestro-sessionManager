# Backend & agent workflow (Maestro)

This document supports the OpenSpec capability **backend-dev-workflow** (`openspec/changes/backend-superpowers-workflow/specs/backend-dev-workflow/spec.md`).

## Scope

Treat **backend** work as changes under **`src-tauri/`**: Rust modules, Tauri commands, `Cargo.toml`, and `tauri.conf.json` when it affects runtime behavior.

## Superpowers skills (read before coding)

Use the **Skill** tool (or open the files) so the full workflow applies—not a paraphrase from memory.

| When | Skill | Path in this repo |
|------|--------|-------------------|
| Any new backend behavior or bugfix | **test-driven-development** | `.agents/skills/test-driven-development/SKILL.md` |
| Before claiming tests pass / work is done | **verification-before-completion** | `.agents/skills/verification-before-completion/SKILL.md` |
| Test failures or confusing runtime behavior | **systematic-debugging** | `.agents/skills/systematic-debugging/SKILL.md` |
| Start of a task / choosing skills | **using-superpowers** | `.agents/skills/using-superpowers/SKILL.md` |

If the Superpowers plugin is installed, equivalent skills may also live under the plugin cache; prefer the repo copies above when paths differ.

## Verification commands

Run these **before** stating that backend work is complete (narrow filters when appropriate):

- **Rust:** `cd src-tauri && cargo test` (or `cargo test <module_filter>`)
- **Frontend or shared types / Tauri bridge:** from repo root, `npm run build`
- **Full desktop smoke:** `npm run tauri dev` (manual)

## Git commits

Prefer **small commits** with clear messages (e.g. `feat(maestro): …`, `fix(src-tauri): …`) after each finished OpenSpec sub-task or logical unit—not one huge commit at the end of a section.

## Related OpenSpec change

- Process & normative spec: `openspec/changes/backend-superpowers-workflow/`

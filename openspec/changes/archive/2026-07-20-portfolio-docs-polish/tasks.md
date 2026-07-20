## 1. License and metadata

- [x] 1.1 Add root `LICENSE` (MIT unless author chooses otherwise at apply time)
- [x] 1.2 Add README/docs note on third-party deps and GPL implications for binaries using Cosmic protocol crates
- [x] 1.3 Fix `src-tauri/Cargo.toml` authors (and description if needed) away from placeholder `you`

## 2. Portfolio README

- [x] 2.1 Rewrite README: problem, launcher-only positioning, stack, quick start, non-goals, links to docs/OpenSpec
- [x] 2.2 Add CI badge pointing at existing workflow
- [x] 2.3 Link architecture and security blurb (inline or `docs/`)

## 3. Screenshots and visuals

- [x] 3.1 Create `docs/screenshots/` (hub required; capture and activation overlay when available)
- [x] 3.2 Embed or link screenshots from README
- [x] 3.3 If captures must wait on overlay, add placeholder note and capture checklist

## 4. Community / expectations

- [x] 4.1 Add short `CONTRIBUTING.md` (personal portfolio project, issue/PR expectations)
- [x] 4.2 Add brief security note (local JSON profiles, no cloud accounts)

## 5. i18n structure

- [x] 5.1 Introduce locale layout (`pt` + `en` stub) and shared accessor; default locale pt-PT
- [x] 5.2 Migrate call sites from direct `pt` imports to the accessor without changing pt copy meaning
- [x] 5.3 Replace misleading **Desactivar**-style copy for label-clear with launcher-honest wording (coordinate with features change if both apply)

## 6. Verification

- [x] 6.1 Confirm LICENSE + README + CONTRIBUTING render correctly on GitHub preview
- [x] 6.2 Frontend build/typecheck after i18n refactor
- [x] 6.3 Update smoke-test checklist pointers if README/docs paths changed

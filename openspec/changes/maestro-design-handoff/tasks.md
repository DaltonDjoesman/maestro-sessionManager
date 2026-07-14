# Tasks: maestro-design-handoff

## 1. OpenSpec & docs

- [x] 1.1 Verify change artifacts (proposal, design, specs) against `design/DESIGN-HANDOFF.md`.
- [x] 1.2 Update `docs/smoke-test-checklist.md` with Captura sidebar route and visual QA viewports.

## 2. Design tokens & primitives (Fase A)

- [x] 2.1 Extract OKLCH tokens from `design/maestro-desktop-prototype.html` into `src/styles/tokens.css`.
- [x] 2.2 Update `src/styles/themes.css` to bridge `[data-theme]` ↔ prototype light/dark sets.
- [x] 2.3 Add `src/styles/components.css` with shared primitives (btn, input, search, card, modal, tabs, badge, checkbox).
- [x] 2.4 Import new stylesheets in `main.tsx` / `App.tsx`; remove obsolete hex token usage.

## 3. Application shell (Fase B)

- [x] 3.1 Add window header (logo, title, decorative controls, theme toggle) to `AppShell.tsx`.
- [x] 3.2 Extend `Sidebar.tsx` nav: Sessões, Captura, Editor (when editing), Definições with SVG icons from prototype.
- [x] 3.3 Add sidebar footer **sessão activa** widget (last activated session + status dot).
- [x] 3.4 Wire theme toggle to existing settings load/save; add route `capture` in `App.tsx`.

## 4. Session hub (Fase C)

- [x] 4.1 Restyle hub toolbar (search, Importar, Nova sessão) per prototype.
- [x] 4.2 Restyle profile cards, sections (Continuar, Fixadas, Todas), badges, hover/focus states.
- [x] 4.3 Implement `ImportProfileModal.tsx` and wire hub Importar action.
- [x] 4.4 Restyle overflow menu (duplicate, export, delete, dry-run).

## 5. Editor & settings (Fase D)

- [x] 5.1 Restyle `ProfileEditorScreen.tsx`: header, tabs, form sections, apps list, browser block, JSON editor.
- [x] 5.2 Restyle `SettingsScreen.tsx` grouped sections and About footer per prototype.
- [x] 5.3 Add missing pt-PT strings in `src/i18n/pt.ts` (Captura, Importar, Desactivar, terminal overlay).

## 6. Capture assistant screen (Fase E)

- [x] 6.1 Create `CaptureAssistantPage.tsx` from prototype capture view (list, filters, create-from-capture).
- [x] 6.2 Reuse existing Tauri capture invoke logic; wire sidebar **Captura** route.
- [x] 6.3 Keep editor **Captura** tab for contextual capture; disable nav/tab when assistant off in settings.

## 7. Activation terminal overlay (Fase F)

- [x] 7.1 Create `ActivationTerminalOverlay.tsx` replacing generic results modal styling.
- [x] 7.2 Wire hub, editor, and dry-run flows to terminal overlay with dry-run indicator.
- [x] 7.3 Preserve **Abrir log** action inside overlay.

## 8. Responsive & verification (Fase G)

- [x] 8.1 Validate layouts at manifest viewports (360, 390, 820, 1024, 1366, 1440, 1920) — no horizontal scroll.
- [x] 8.2 Run `npm run build` and `cargo test`; manual smoke per updated checklist.
- [x] 8.3 Visual comparison screenshots against `design/maestro-desktop-prototype.html` at 1440×900.

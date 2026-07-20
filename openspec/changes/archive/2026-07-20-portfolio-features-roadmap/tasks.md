## 1. Launcher-honest shell (P1)

- [x] 1.1 Update sidebar footer copy/behavior so clear action only resets local “last activated” state (no teardown implication)
- [x] 1.2 Align i18n strings for the footer with launcher-only wording
- [x] 1.3 Smoke: activate → indicator shows → clear → apps still running

## 2. Dry-run UI (P1)

- [x] 2.1 Add hub and/or editor control invoking `preview_session_activation`
- [x] 2.2 Present planned steps (label + argv); reuse results overlay in preview mode if available
- [x] 2.3 Verify preview does not spawn processes

## 3. First-run empty hub (P1)

- [x] 3.1 Empty-state UI when catalog has zero profiles (explanation + CTA)
- [x] 3.2 Wire “create from example” or clear path to `docs/examples/` sample profile
- [x] 3.3 Keep empty state from blocking **Nova sessão** / **Importar**

## 4. Import / export modals (P1)

- [x] 4.1 Replace prompt-based import with dedicated import modal (file + display name + errors)
- [x] 4.2 Dedicated export flow (modal and/or save dialog) from hub overflow / editor
- [x] 4.3 Preserve schema validation and filename safety rules

## 5. Multi-distro packaging (P2)

- [x] 5.1 Document deb + AppImage build steps and distro dependency families (Debian/Ubuntu + Fedora or Arch)
- [x] 5.2 Verify or adjust `tauri.conf.json` bundle targets; note host limitations
- [x] 5.3 UI/docs copy when workspace grouping is unavailable on unsupported compositors

## 6. Compositor adapters (P3–P4)

- [x] 6.1 Spike + implement GNOME Wayland discovery adapter (best-effort) behind platform boundary
- [x] 6.2 Best-effort KWin and/or Hyprland adapter or documented no-op with degradation
- [x] 6.3 Tests/fixtures for adapter isolation (no Cosmic types in shared DTOs)

## 7. Optional craft (P4)

- [x] 7.1 CLI `maestro activate <profile>` (or document as follow-up change)
- [x] 7.2 Optional global hotkey research / stub
- [x] 7.3 Profile templates (“web dev”, etc.) if time permits

## 8. Explicit non-goals check

- [x] 8.1 Confirm roadmap/docs do not promise open-tab URL import or workspace window placement
- [x] 8.2 Confirm no context-cleanup / process teardown reintroduced

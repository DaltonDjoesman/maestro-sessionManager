## Context

`wayland-app-discovery` shipped foreign-toplevel listing on Cosmic: titles/`app_id` work, supplemental `wmctrl` for XWayland, and Tier A scoring. Cosmic workspace attachment was intentionally soft-failed in `try_attach_cosmic_workspace_metadata` — spike confirmed unprivileged clients can bind foreign-toplevel and Cosmic advertises `zcosmic_toplevel_info_v1`, but handle→index mapping needs Cosmic workspace protocol wiring.

Today `handle_info_to_record` uses `desktop: info.desktop.unwrap_or(0)`, and window-first discovery always sets `desktopWorkspace: Some(window.desktop)`. Result: every Cosmic native window looks like workspace index `0` → UI shows one “Workspace 1” bucket. X11 still gets real indices from `wmctrl` (`_NET_WM_DESKTOP`).

## Goals / Non-Goals

**Goals:**

- Attach real Cosmic workspace indices to Wayland `WindowRecord`s so the assistant groups by workspace.
- Soft-fail cleanly when Cosmic protocols are unavailable — omit unknown workspace rather than faking index `0`.
- Keep X11/`wmctrl` and IPC/UI contracts intact.
- Test mapping logic with fixtures (no live compositor in CI); document manual Cosmic verification.

**Non-Goals:**

- Window management (move/focus/resize) or Cosmic panel/launcher behavior.
- Full multi-compositor workspace adapters (GNOME/KWin/Hyprland).
- Pulling `libcosmic` into the Tauri crate.
- Redesigning the frontend workspace grouping UI.

## Decisions

### 1. Implement Cosmic attach inside the existing short-lived Wayland client

Extend the snapshot in `wayland_windows.rs` (same connect → roundtrip → disconnect, ~750ms timeout) to also bind Cosmic toplevel-info and workspace globals when present. After foreign-toplevel handles are collected, enrich each record’s workspace field via Cosmic events, then return.

**Alternative considered:** Separate second Wayland connection only for Cosmic → rejected; doubles connect cost and complicates matching handles to records.

**Alternative considered:** Long-lived dock-style listener → rejected; capture refresh is a one-shot command.

### 2. Prefer `cosmic-protocols` (client) over `libcosmic`

Add `cosmic-protocols` with the `client` feature (or generate thin bindings from the same XML) alongside existing `wayland-client` / `wayland-protocols`. Use `zcosmic_toplevel_info_v1` workspace enter/leave (or `ext_workspace_enter` when that is what the compositor emits) plus Cosmic/`ext` workspace manager events to build a handle→stable 0-based index map.

**Note:** `cosmic-protocols` is GPL-3.0; confirm license compatibility with this crate before landing the dependency. If blocked, vendor only the needed protocol XML + `wayland-scanner` codegen under the project’s license policy.

**Alternative considered:** `libcosmic` / CCTK → rejected; too heavy for a snapshot helper.

**Alternative considered:** Shell helper binary → deferred unless in-process binding fails on real Cosmic after wiring.

### 3. Stable 0-based index from workspace enumeration order

Assign indices by the order Cosmic announces workspaces during the snapshot (first seen = `0`, next = `1`, …), matching the existing EWMH “desktop number” mental model the UI already uses (“Workspace 1” = index `0` + 1). If a toplevel is on multiple workspaces, pick one deterministic rule (prefer the first `workspace_enter` / lowest index) and document it.

Match Cosmic toplevel handles to foreign-toplevel records by the protocol’s foreign-toplevel association when available; otherwise fall back to title/`app_id` equality for the snapshot duration.

### 4. Unknown workspace must not emit fake `0`

Change the Wayland path so unresolved workspace leaves `desktopWorkspace` unset:

- Prefer `WindowRecord.desktop: Option<u32>` (or an explicit `workspace_known` flag) for Wayland-sourced rows; `wmctrl` rows keep concrete indices.
- Stop using `unwrap_or(0)` as “known workspace” for foreign-toplevel.
- Soft-fail Cosmic attach → leave workspace unset → UI falls back to flat / “No workspace” per `desktop-workspace-hints`, not a single fake “Workspace 1”.

**Alternative considered:** Sentinel `u32::MAX` → rejected; easy to leak into UI headings.

### 5. Soft-fail and logging stay best-effort

Any bind failure, timeout, or partial Cosmic state logs a short stderr/debug line and continues with titles/`app_id` only. The assistant command never fails solely because Cosmic workspace failed.

### 6. Docs + fixture tests

Update `docs/linux-desktop.md` Cosmic section: workspace grouping works when Cosmic grants toplevel-info/workspace protocols; manual check = apps on workspace 1 and 2 → distinct headings.

Unit-test pure mapping helpers with synthetic handle/event fixtures; keep the existing soft-fail test updated to expect “unchanged / None” rather than forced `0`. Live Cosmic check remains manual.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Cosmic denies toplevel-info or workspace globals to unprivileged clients | Soft-fail; omit `desktopWorkspace`; titles still list; document |
| Protocol version drift (`workspace_enter` deprecated for `ext_workspace_enter`) | Prefer events Cosmic actually emits on current Pop; support both if cheap |
| GPL `cosmic-protocols` vs project license | Check before merge; fallback to scanner + vendored XML |
| Multi-workspace toplevels (sticky) | Deterministic single-index pick; document |
| Matching Cosmic handle ↔ foreign-toplevel record | Prefer protocol association; title/`app_id` fallback only within one snapshot |
| CI cannot run Cosmic | Fixture unit tests only; manual verification checklist in docs |

## Migration Plan

1. Land dependency + workspace index mapping helpers with fixture tests.
2. Wire Cosmic bind/events into the existing Wayland snapshot; fix unknown-vs-0 semantics end-to-end.
3. Update docs; manually verify on Cosmic (two workspaces, multi-window browser).
4. Rollback: restore soft-noop attach + previous `unwrap_or(0)` only if needed; discovery titles remain from foreign-toplevel.

## Open Questions

1. Exact Cosmic globals on current Pop!_OS (Cosmic workspace manager vs `ext-workspace-v1` only) — confirm during implement spike against live registry.
2. Does `zcosmic_toplevel_info_v1` associate each Cosmic handle with an `ext_foreign_toplevel_handle_v1` object id we can key on, or is title/`app_id` matching required?

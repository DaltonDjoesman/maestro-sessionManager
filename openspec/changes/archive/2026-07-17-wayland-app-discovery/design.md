## Context

Maestro’s capture assistant discovers running apps via a **window-first** path (`wmctrl -l -p` on X11) and a **process + `.desktop` fallback**. On Pop!_OS Cosmic the session is now pure Wayland (`XDG_SESSION_TYPE=wayland`). The workspace loader **skips** `wmctrl` on Wayland, so discovery falls back to processes. The classifier then applies `SCORE_NO_WINDOW_PENALTY` (−50) to desktop matches without a mapped window, leaving scores below the `app` threshold (80). The UI filters to `kind === "app"`, so the list looks empty.

Cosmic does **not** expose a stable D-Bus window list. It uses Wayland protocols (`ext-foreign-toplevel-list-v1`, Cosmic `zcosmic_toplevel_info_v1`) for docks/launchers. Access may be privileged. XWayland apps still appear under EWMH/`wmctrl` even on Wayland hosts (partial coverage only).

## Goals / Non-Goals

**Goals:**

- Restore a usable capture list on Cosmic Wayland (and other Wayland sessions) without regressing X11.
- Prefer window-anchored discovery whenever any window source returns mapped surfaces (native Wayland and/or XWayland).
- Make strong Freedesktop `.desktop` matches classify as `app` on Wayland when no window source is available, while still excluding denylisted noise.
- Attach `desktopWorkspace` when Cosmic (or another adapter) provides it; otherwise keep flat grouping.
- Document session expectations and manual verification in `docs/linux-desktop.md`.

**Non-Goals:**

- Moving, focusing, or resizing windows (unchanged product constraint).
- Full multi-compositor parity in one change (GNOME Shell Introspect, KWin, Hyprland are optional later adapters).
- Replacing the scored classifier with a basename allowlist.
- Guaranteeing privileged Wayland protocol access on every Cosmic install without a documented fallback.

## Decisions

### 1. Two-tier delivery: scoring fix first, then Wayland window adapter

| Tier | What | Why |
|------|------|-----|
| **A (must ship)** | Adjust Wayland scoring / “no window” semantics so strong `.desktop` matches become `app` when window enumeration is unavailable | Unblocks capture immediately on Cosmic; no new compositor dependency |
| **B (best-effort)** | Add a Wayland window source using `ext-foreign-toplevel-list-v1` (+ Cosmic toplevel info for workspace when available) behind a platform adapter | Restores window-first quality (titles, multi-window rows, workspace) when the compositor grants the protocol |

**Alternative considered:** Cosmic D-Bus only → rejected; no usable well-known window API on this host.

**Alternative considered:** Wayland-only protocol work before scoring fix → rejected; protocol privilege may fail; users stay blocked.

### 2. Pluggable `WindowSource` behind `WorkspaceIndex`

Refactor `WorkspaceIndex::load()` to merge records from:

1. **X11 / EWMH** — `wmctrl -l -p` when useful (session `x11`/`tty`, **and** on Wayland as a supplemental XWayland source when `wmctrl` succeeds).
2. **Wayland foreign toplevel** — best-effort client; map `app_id` / title / optional PID / workspace into existing `WindowRecord` (PID may be missing → keep title/desktop matching paths already used for bogus PIDs).

Empty merge → process fallback as today.

**Alternative considered:** Always prefer process scan on Wayland → rejected; loses multi-window and workspace quality when protocols work.

### 3. Classifier: session-aware “no window” rule

Today: desktop match without window → −50 → score 40 → `process` (intentional to hide background services with `.desktop` files).

Change:

- When a **window source is active** and returned a non-empty list: keep current rule (no mapped surface → prefer `process`).
- When **no window source produced windows** (typical pure Wayland without protocol access): **do not** apply the no-window penalty for strong desktop matches that already pass noise filters; score must meet `APP_THRESHOLD` (e.g. desktop + session ≥ 80).

Noise denylist / chromium subprocess / assistant filters remain the primary daemon exclusion.

**Alternative considered:** Always drop the penalty → riskier on X11 where window list exists and daemons would inflate the list.

### 4. Dependency choice for Wayland protocols

Prefer a **small, isolated Rust Wayland client** (or thin wrapper around `wayland-client` + protocol crates) used only during capture refresh, not a long-lived dock client. If binding fails or returns empty, log at debug and continue with Tier A.

Avoid `libcosmic` as a hard dependency of the Tauri crate unless a spike proves it’s the only practical path; keep Cosmic-specific code behind `cfg(target_os = "linux")` modules.

**Alternative considered:** Shell out to a helper binary → acceptable fallback if in-process binding is blocked; document in tasks if spike chooses it.

### 5. IPC / UI contract unchanged

`list_assistant_running_apps` and `RunningAppCandidate` fields stay the same. Frontend `filterByKind` / workspace grouping need no redesign once kinds and optional `desktopWorkspace` are correct.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Cosmic denies foreign-toplevel to unprivileged apps | Tier A scoring always restores a usable list; document limitation; optional future portal/privilege story |
| XWayland-only `wmctrl` on Wayland shows a subset of apps | Merge with process+desktop / foreign-toplevel; never treat partial wmctrl as complete coverage alone |
| False-positive “apps” without windows (daemons with `.desktop`) | Keep denylist + background-noise filters; only waive penalty when window list is unavailable |
| Wayland client complexity / event-loop friction inside Tauri command | Short-lived connect → snapshot → disconnect; timeout; never block UI forever |
| Multi-compositor divergence | Adapter trait + Cosmic-first implementation; others return empty |

## Migration Plan

1. Land Tier A (classifier + tests + docs) so Cosmic users get apps back immediately.
2. Land `WindowSource` merge + optional wmctrl-on-Wayland for XWayland.
3. Land Wayland foreign-toplevel adapter behind feature or soft-fail.
4. Manual verify on Cosmic: open native Wayland apps + one XWayland app; refresh capture; confirm `app` rows and optional workspace sections.
5. Rollback: revert adapter modules; Tier A alone remains an improvement over current empty UI.

## Open Questions

1. Does Cosmic grant `ext-foreign-toplevel-list-v1` to a normal user Tauri app today, or only to privileged session clients (panel/launcher)? Spike must answer before committing to in-process vs helper.
2. Can Cosmic toplevel info reliably supply workspace index for grouping, or only title/`app_id`?
3. Should GNOME (`org.gnome.Shell.Introspect`) be a follow-up change or a thin second adapter in this one if time allows? **Default: follow-up.**

//! Compositor discovery adapters behind the Wayland / workspace boundary.
//!
//! Shared capture DTOs only see portable [`crate::platform::WindowRecord`] fields
//! (`pid`, optional `desktop`, `title`, optional `app_id`). Cosmic protocol types
//! stay inside `wayland_windows` and must not leak into profiles or assistant DTOs.
//!
//! | Compositor | Window list | Workspace index |
//! |------------|-------------|-----------------|
//! | Cosmic     | foreign-toplevel + Cosmic enrichment | best-effort via Cosmic protocols |
//! | GNOME      | foreign-toplevel when Mutter advertises it | no-op (omit `desktop`) |
//! | KWin       | foreign-toplevel when available | no-op (omit `desktop`) |
//! | Hyprland   | foreign-toplevel when available | documented no-op (omit `desktop`) |
//! | X11        | `wmctrl` (see `workspace`) | EWMH indices |

/// High-level desktop environment hint for logging / future adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositorFamily {
    Cosmic,
    Gnome,
    Kwin,
    Hyprland,
    Unknown,
}

/// Detect a coarse compositor family from common environment variables.
pub fn detect_compositor_family() -> CompositorFamily {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_ascii_lowercase();
    let session = std::env::var("XDG_SESSION_DESKTOP")
        .unwrap_or_default()
        .to_ascii_lowercase();
    let combined = format!("{desktop};{session}");

    if combined.contains("cosmic") {
        CompositorFamily::Cosmic
    } else if combined.contains("gnome") {
        CompositorFamily::Gnome
    } else if combined.contains("kde") || combined.contains("plasma") {
        CompositorFamily::Kwin
    } else if combined.contains("hyprland") {
        CompositorFamily::Hyprland
    } else {
        CompositorFamily::Unknown
    }
}

/// Whether Maestro expects native workspace enrichment for this family.
///
/// `false` means: keep foreign-toplevel / process discovery; leave `desktop` unset
/// rather than inventing index `0`.
pub fn workspace_enrichment_expected(family: CompositorFamily) -> bool {
    matches!(family, CompositorFamily::Cosmic)
}

/// GNOME Wayland: titles/`app_id` via shared foreign-toplevel path when Mutter
/// advertises `ext-foreign-toplevel-list-v1`. Workspace membership is not exposed
/// to unprivileged clients in a portable way — adapter is intentionally a no-op
/// for `desktop` enrichment (best-effort discovery only).
#[allow(dead_code)]
pub fn gnome_workspace_enrichment_available() -> bool {
    false
}

/// KWin / Plasma Wayland: same policy — no fabricated workspace indices.
#[allow(dead_code)]
pub fn kwin_workspace_enrichment_available() -> bool {
    false
}

/// Hyprland: documented no-op for workspace indices (hyprctl could be spiked later).
#[allow(dead_code)]
pub fn hyprland_workspace_enrichment_available() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::workspace::{merge_window_sources, WindowRecord, WindowSource};

    #[test]
    fn portable_window_record_has_no_cosmic_protocol_fields() {
        // Compile-time / structural isolation: WindowRecord is only portable fields.
        let r = WindowRecord {
            pid: 42,
            desktop: None,
            title: "Example".into(),
            app_id: Some("org.example.App".into()),
        };
        assert!(r.desktop.is_none());
        assert_eq!(r.app_id.as_deref(), Some("org.example.App"));
        let _ = format!("{r:?}");
    }

    #[test]
    fn gnome_and_tilers_do_not_claim_workspace_enrichment() {
        assert!(!gnome_workspace_enrichment_available());
        assert!(!kwin_workspace_enrichment_available());
        assert!(!hyprland_workspace_enrichment_available());
        assert!(!workspace_enrichment_expected(CompositorFamily::Gnome));
        assert!(!workspace_enrichment_expected(CompositorFamily::Kwin));
        assert!(!workspace_enrichment_expected(CompositorFamily::Hyprland));
        assert!(workspace_enrichment_expected(CompositorFamily::Cosmic));
    }

    #[test]
    fn workspace_index_accepts_records_without_desktop() {
        let index = merge_window_sources([WindowSource::WaylandForeignToplevel(vec![
            WindowRecord {
                pid: 100,
                desktop: None,
                title: "No WS".into(),
                app_id: Some("app".into()),
            },
        ])]);
        assert_eq!(index.windows().len(), 1);
        assert!(index.workspace_for_pid(100).is_none());
    }
}

//! OS-specific adapters (process listing). MVP targets Linux only.

mod linux;
mod wayland_windows;
mod workspace;

pub(crate) use workspace::{uses_window_first_discovery, WindowRecord, WorkspaceIndex};

pub(crate) use linux::{denylisted_basename, has_resolved_executable};

use serde::Serialize;

/// One user-visible process row (Linux: post-filters for tooling like the profile assistant).
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessCandidate {
    pub pid: u32,
    /// Lowercased executable basename (from `exe` when available, else process name).
    pub executable_basename: String,
    /// Short human-readable command preview for the UI.
    pub cmd_preview: String,
}

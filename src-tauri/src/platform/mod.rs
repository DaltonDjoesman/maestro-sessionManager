//! OS-specific adapters (process listing, signals). MVP targets Linux only.

mod linux;

pub use linux::LinuxPlatform;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("platform operation failed: {0}")]
    Operation(String),
}

/// One user-visible process row for cleanup diff (Linux: post-filters).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessCandidate {
    pub pid: u32,
    /// Lowercased executable basename (from `exe` when available, else process name).
    pub executable_basename: String,
    /// Short human-readable command preview for the UI.
    pub cmd_preview: String,
}

/// Cross-platform surface for process and signal operations.
pub trait PlatformContext: Send + Sync {
    fn platform_name(&self) -> &'static str;

    /// Processes considered for cleanup divergence (noise and denylist already removed).
    fn list_cleanup_process_candidates(&self) -> Result<Vec<ProcessCandidate>, PlatformError>;
}

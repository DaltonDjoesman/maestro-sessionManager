//! OS-specific adapters (process listing, signals). MVP targets Linux only.

mod linux;

pub use linux::LinuxPlatform;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("platform operation failed: {0}")]
    Operation(String),
}

/// Cross-platform surface for process and signal operations.
pub trait PlatformContext: Send + Sync {
    fn platform_name(&self) -> &'static str;
}

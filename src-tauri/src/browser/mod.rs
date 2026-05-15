//! Browser argv builders for Chromium-like and Firefox families (`browser-launch` spec).

mod build;

pub use build::{build_chromium_like_command, BuiltBrowserCommand};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrowserError {
    #[error("browser launch error: {0}")]
    Message(String),
}

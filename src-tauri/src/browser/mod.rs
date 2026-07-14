//! Browser argv builders for Chromium-like and Firefox families (`browser-launch` spec).
//!
//! Use [`build_browser_launch`] for activation: argv preserves HTTPS and `file://` URLs while
//! [`BuiltBrowserCommand::warnings`] collects isolation and sandbox hints for the summary (task 5.3).

mod build;
mod detect;
mod hint;

pub use build::{
    build_browser_launch, build_chromium_like_command, build_firefox_command, BuiltBrowserCommand,
};
pub use detect::{detect_browser_family, executable_basename, is_browser_executable};
pub use hint::{detect_system_default_browser, SystemDefaultBrowserHint};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrowserError {
    #[error("browser launch error: {0}")]
    Message(String),
}

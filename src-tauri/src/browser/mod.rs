//! Browser command-line builders for Chromium-like and Firefox families (task 5).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrowserError {
    #[error("browser launch error: {0}")]
    Message(String),
}

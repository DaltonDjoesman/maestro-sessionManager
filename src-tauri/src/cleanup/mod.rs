//! Context cleanup: process diff, confirmation, SIGTERM/SIGKILL (task 7).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CleanupError {
    #[error("cleanup error: {0}")]
    Message(String),
}

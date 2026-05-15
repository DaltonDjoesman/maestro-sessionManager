//! Global application settings persisted on disk (implemented in task 2).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("settings error: {0}")]
    Message(String),
}

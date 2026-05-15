//! Global application settings persisted on disk (task 2).

mod defaults;
mod model;

pub use defaults::{default_profiles_root, default_settings, maestro_data_dir};
pub use model::{
    ApplicationSettings, BrowserFamily, LogVerbosity, UiTheme, CURRENT_SCHEMA_VERSION,
    SUPPORTED_SCHEMA_VERSION,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("settings error: {0}")]
    Message(String),
    #[error("could not resolve Maestro data directory: {0}")]
    DataDirUnavailable(String),
}

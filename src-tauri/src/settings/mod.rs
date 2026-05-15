//! Global application settings persisted on disk (task 2).

mod defaults;
mod model;
mod store;

pub use defaults::{default_profiles_root, default_settings, maestro_data_dir};
pub use model::{
    ApplicationSettings, BrowserFamily, LogVerbosity, UiTheme, CURRENT_SCHEMA_VERSION,
    SUPPORTED_SCHEMA_VERSION,
};
pub use store::{settings_file_path, SettingsStore};

use thiserror::Error;

/// Loaded settings plus store handle for persistence (managed by Tauri).
pub struct SettingsManager {
    store: SettingsStore,
    pub settings: ApplicationSettings,
}

impl SettingsManager {
    pub fn open_default() -> Result<Self, SettingsError> {
        let store = SettingsStore::open_default()?;
        let settings = store.load()?;
        Ok(Self { store, settings })
    }

    pub fn persist(&self) -> Result<(), SettingsError> {
        self.store.save(&self.settings)
    }

    pub fn persist_if_absent(&self) -> Result<bool, SettingsError> {
        self.store.save_if_absent(&self.settings)
    }
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("settings error: {0}")]
    Message(String),
    #[error("could not resolve Maestro data directory: {0}")]
    DataDirUnavailable(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid settings JSON: {0}")]
    InvalidJson(String),
    #[error(
        "unsupported settings schema_version {found} (this build supports up to {supported})"
    )]
    UnsupportedSchemaVersion { found: u32, supported: u32 },
}

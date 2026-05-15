//! Global application settings persisted on disk (task 2).

mod defaults;
mod model;
mod store;
mod validation;

pub use defaults::{default_profiles_root, default_settings, maestro_data_dir};
pub use model::{
    ApplicationSettings, BrowserFamily, LogVerbosity, UiTheme, CURRENT_SCHEMA_VERSION,
    SUPPORTED_SCHEMA_VERSION,
};
pub use store::{settings_file_path, SettingsStore};
pub use validation::{validate_profiles_root_path, ProfilesRootValidationError};

use std::fs;
use std::sync::Mutex;

use thiserror::Error;

/// Loaded settings plus store handle for persistence (managed by Tauri).
pub struct SettingsManager {
    store: SettingsStore,
    settings: Mutex<ApplicationSettings>,
}

impl SettingsManager {
    pub fn open_default() -> Result<Self, SettingsError> {
        let store = SettingsStore::open_default()?;
        let settings = store.load()?;
        Ok(Self {
            store,
            settings: Mutex::new(settings),
        })
    }

    pub fn get(&self) -> ApplicationSettings {
        self.settings.lock().expect("settings lock").clone()
    }

    pub fn update_and_save(
        &self,
        mut new_settings: ApplicationSettings,
    ) -> Result<ApplicationSettings, SettingsError> {
        let path = new_settings.profiles_root_path();
        validate_profiles_root_path(&path)?;

        if !path.exists() {
            fs::create_dir_all(&path).map_err(SettingsError::Io)?;
        }

        new_settings.schema_version = CURRENT_SCHEMA_VERSION;

        if !new_settings.is_schema_supported() {
            return Err(SettingsError::UnsupportedSchemaVersion {
                found: new_settings.schema_version,
                supported: SUPPORTED_SCHEMA_VERSION,
            });
        }

        self.store.save(&new_settings)?;
        *self.settings.lock().expect("settings lock") = new_settings.clone();
        Ok(new_settings)
    }

    pub fn persist(&self) -> Result<(), SettingsError> {
        let settings = self.get();
        self.store.save(&settings)
    }

    pub fn persist_if_absent(&self) -> Result<bool, SettingsError> {
        let settings = self.get();
        self.store.save_if_absent(&settings)
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
    #[error(transparent)]
    Validation(#[from] ProfilesRootValidationError),
}

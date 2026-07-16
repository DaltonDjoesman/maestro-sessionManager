//! Session profiles on disk: JSON schema, catalog listing, CRUD, atomic saves.

mod model;
mod service;

pub use model::{ApplicationLaunchEntry, ProfileBrowserBlock, SessionProfile};

#[allow(unused_imports)] // Re-exported for tests and future command surface.
pub use model::{
    ApplicationBrowserSettings, ProfileCleanupRules, CURRENT_PROFILE_SCHEMA_VERSION,
    SUPPORTED_PROFILE_SCHEMA_VERSION,
};
pub use service::{
    CreateProfileResult, DuplicateProfileResult, ProfileCatalogEntry, ProfileDirectory,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid profile JSON at {path}: {message}")]
    InvalidJson { path: String, message: String },
    #[error("profile {path}: unsupported schema_version {found} (supported up to {supported})")]
    UnsupportedSchema {
        path: String,
        found: u32,
        supported: u32,
    },
    #[error("profile {path}: {message}")]
    Validation { path: String, message: String },
    #[error("path outside profiles root")]
    PathOutsideRoot,
    #[error("{0}")]
    RootUnavailable(String),
}

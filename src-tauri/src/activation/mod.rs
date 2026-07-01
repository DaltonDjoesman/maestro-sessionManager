//! Session activation orchestration: validate profile, launch browser and apps (task 6).

mod log;
mod plan;
mod runner;
pub(crate) mod skip;
mod summary;
mod validate;

pub use plan::ActivationPreviewStep;
pub use runner::{activate_session_profile, preview_session_activation};
pub use summary::{
    ActivateSessionResult, ActivationStepKind, ActivationStepStatus, ActivationStepSummary,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActivationError {
    #[error(transparent)]
    Profile(#[from] crate::profiles::ProfileError),
    #[error(transparent)]
    Browser(#[from] crate::browser::BrowserError),
}

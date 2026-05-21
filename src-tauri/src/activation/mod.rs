//! Session activation orchestration: validate profile, launch browser and apps (task 6).

mod runner;
pub(crate) mod skip;
mod summary;
mod validate;

pub use runner::activate_session_profile;
pub use summary::{ActivationStepKind, ActivationStepStatus, ActivationStepSummary};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActivationError {
    #[error(transparent)]
    Profile(#[from] crate::profiles::ProfileError),
    #[error(transparent)]
    Browser(#[from] crate::browser::BrowserError),
}

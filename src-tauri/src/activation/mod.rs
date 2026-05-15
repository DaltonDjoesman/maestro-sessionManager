//! Session activation orchestration: validate profile, launch browser and apps (task 6).

mod summary;

pub use summary::{ActivationStepKind, ActivationStepStatus, ActivationStepSummary};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActivationError {
    #[error("activation error: {0}")]
    Message(String),
}

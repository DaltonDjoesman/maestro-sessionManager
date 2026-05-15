//! Session profile JSON: load, validate, CRUD (implemented in task 3).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("profile error: {0}")]
    Message(String),
}

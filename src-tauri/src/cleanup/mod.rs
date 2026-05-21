//! Context cleanup: divergence vs profile, confirmation-driven termination (`context-cleanup` spec).

mod diff;
mod terminate;
mod types;

pub use diff::{allowed_executable_basenames, compute_divergences};
pub use terminate::terminate_process;
pub use types::{CleanupDivergenceRow, CleanupTerminateResult};

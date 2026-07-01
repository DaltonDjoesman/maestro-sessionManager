//! Assisted profile capture: running-process suggestions for the profile editor.

mod assistant;
mod classifier;
mod desktop_index;
mod window_discovery;

pub use assistant::{list_running_app_candidates, RunningAppCandidate};

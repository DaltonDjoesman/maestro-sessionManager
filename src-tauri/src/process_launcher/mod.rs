//! Non-blocking process launches from profile-style entries (`process-launcher` spec).

mod sequence;
mod spawn;
mod summary;

pub use sequence::spawn_ordered_with_delay_after_success;
pub use spawn::{spawn_command, spawn_launch_spec, LaunchSpec, SpawnOutcome};
pub use summary::{map_application_spawn_results, spawn_application_sequence_summaries};

use std::path::PathBuf;

use crate::profiles::ApplicationLaunchEntry;

impl From<&ApplicationLaunchEntry> for LaunchSpec {
    fn from(entry: &ApplicationLaunchEntry) -> Self {
        Self {
            executable: entry.executable.clone(),
            args: entry.args.clone(),
            cwd: entry.cwd.as_ref().map(PathBuf::from),
        }
    }
}

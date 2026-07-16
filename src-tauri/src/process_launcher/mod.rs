//! Non-blocking process launches from profile-style entries (`process-launcher` spec).

mod sequence;
mod spawn;
mod summary;

pub use spawn::{resolved_spawn_argv, spawn_command, spawn_launch_spec, LaunchSpec, SpawnOutcome};

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

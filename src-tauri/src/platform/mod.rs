//! OS-specific adapters (process listing). MVP targets Linux only.

mod linux;
mod wayland_windows;
mod workspace;

pub(crate) use workspace::{uses_window_first_discovery, WindowRecord, WorkspaceIndex};

pub(crate) use linux::{
    denylisted_basename, has_resolved_executable, resolve_launch_executable,
    resolve_process_executable, strip_deleted_exe_suffix,
};

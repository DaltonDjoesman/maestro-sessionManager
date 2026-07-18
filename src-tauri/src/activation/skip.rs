//! Skip-if-already-running using executable basename vs running processes (`session-activation` spec).

use std::collections::HashSet;
use std::path::Path;

use sysinfo::{ProcessesToUpdate, System};

use crate::editors::is_known_editor_basename;
use crate::profiles::ApplicationLaunchEntry;

/// Lowercased basename used for matching (e.g. `/usr/bin/Cursor` → `cursor`).
pub fn executable_basename(executable: &str) -> String {
    let t = crate::platform::strip_deleted_exe_suffix(executable.trim());
    Path::new(t)
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| t.to_lowercase())
}

/// Snapshot of lowercased executable basenames currently running on this machine.
pub fn collect_running_executable_basenames() -> HashSet<String> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    let mut set = HashSet::new();
    for proc in sys.processes().values() {
        let from_exe = proc.exe().and_then(|p| {
            let owned = p.to_string_lossy().into_owned();
            let cleaned = crate::platform::strip_deleted_exe_suffix(&owned);
            Path::new(cleaned)
                .file_name()
                .map(|n| n.to_string_lossy().to_lowercase())
        });
        let key = from_exe.unwrap_or_else(|| proc.name().to_string_lossy().to_lowercase());
        if !key.is_empty() {
            set.insert(key);
        }
    }
    set
}

/// When `skip_if_running` is true for the entry and a matching basename is running, return a UI detail string.
pub fn entry_skip_detail(entry: &ApplicationLaunchEntry, running: &HashSet<String>) -> Option<String> {
    if !matches!(entry.skip_if_running, Some(true)) {
        return None;
    }
    if editor_should_relaunch_with_target(entry) {
        return None;
    }
    let b = executable_basename(&entry.executable);
    if running.contains(&b) {
        Some(format!(
            "skipped: executable basename `{b}` is already running"
        ))
    } else {
        None
    }
}

fn editor_should_relaunch_with_target(entry: &ApplicationLaunchEntry) -> bool {
    let base = executable_basename(&entry.executable);
    if !is_known_editor_basename(&base) {
        return false;
    }
    entry.args.iter().any(|arg| {
        let arg = arg.trim();
        !arg.starts_with('-') && Path::new(arg).is_absolute()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basename_from_path_is_file_component_lower() {
        assert_eq!(executable_basename("/usr/bin/Foo"), "foo");
    }

    #[test]
    fn basename_plain_name_lower() {
        assert_eq!(executable_basename("Cursor"), "cursor");
    }

    #[test]
    fn skip_detail_when_policy_and_match() {
        let entry = ApplicationLaunchEntry {
            executable: "/bin/sh".into(),
            args: vec![],
            cwd: None,
            skip_if_running: Some(true),
            browser: None,
        };
        let mut set = HashSet::new();
        set.insert("sh".into());
        let d = entry_skip_detail(&entry, &set).expect("skip");
        assert!(d.contains("sh"));
    }

    #[test]
    fn no_skip_editor_with_folder_target_when_running() {
        let entry = ApplicationLaunchEntry {
            executable: "/usr/share/cursor/cursor".into(),
            args: vec![
                "--reuse-window".into(),
                "/home/user/openspectutorial".into(),
            ],
            cwd: Some("/home/user/openspectutorial".into()),
            skip_if_running: Some(true),
            browser: None,
        };
        let mut set = HashSet::new();
        set.insert("cursor".into());
        assert!(entry_skip_detail(&entry, &set).is_none());
    }

    #[test]
    fn no_skip_when_policy_off() {
        let entry = ApplicationLaunchEntry {
            executable: "/bin/sh".into(),
            args: vec![],
            cwd: None,
            skip_if_running: None,
            browser: None,
        };
        let mut set = HashSet::new();
        set.insert("sh".into());
        assert!(entry_skip_detail(&entry, &set).is_none());
    }
}

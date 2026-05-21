use std::ffi::OsStr;

use sysinfo::{
    ProcessesToUpdate, ProcessRefreshKind, System, UpdateKind,
};

use super::{PlatformContext, PlatformError, ProcessCandidate};

/// Linux adapter (Pop!_OS reference). Uses `sysinfo` for process enumeration.
pub struct LinuxPlatform;

impl LinuxPlatform {
    pub fn new() -> Result<Self, PlatformError> {
        Ok(Self)
    }
}

impl PlatformContext for LinuxPlatform {
    fn platform_name(&self) -> &'static str {
        "linux"
    }

    fn list_cleanup_process_candidates(&self) -> Result<Vec<ProcessCandidate>, PlatformError> {
        Ok(list_cleanup_process_candidates_linux())
    }
}

/// Built-in basenames and patterns excluded from user-facing cleanup diff.
fn denylisted_basename(b: &str) -> bool {
    let b = b.trim();
    if b.is_empty() {
        return true;
    }
    const EXACT: &[&str] = &[
        "systemd",
        "systemd-udevd",
        "systemd-journald",
        "systemd-resolved",
        "systemd-timesyncd",
        "systemd-logind",
        "systemd-hostnamed",
        "(sd-pam)",
        "sd-pam",
        "dbus-daemon",
        "dbus-broker",
        "dbus-broker-launch-helper",
        "polkitd",
        "accounts-daemon",
        "rsyslogd",
        "agetty",
        "login",
        "sshd",
        "cupsd",
        "NetworkManager",
        "wpa_supplicant",
        "ModemManager",
        "udisksd",
        "upowerd",
        "colord",
        "gnome-shell",
        "Xorg",
        "xorg",
        "pipewire",
        "pipewire-pulse",
        "wireplumber",
        "pulseaudio",
        "cups-browsed",
        "avahi-daemon",
        "cron",
        "crond",
        "kthreadd",
        "khungtaskd",
        "oom_reaper",
        "migration",
        "watchdogd",
        "jbd2",
    ];
    let lower = b.to_lowercase();
    if EXACT.iter().any(|&x| x.eq_ignore_ascii_case(lower.as_str())) {
        return true;
    }
    if lower.starts_with("kworker") || lower.starts_with("ksoftirq") {
        return true;
    }
    if lower.starts_with("irq/") || lower.starts_with("rcu") {
        return true;
    }
    if lower.starts_with("jbd2/") {
        return true;
    }
    false
}

fn cmd_preview(cmd: &[std::ffi::OsString]) -> String {
    let joined: String = cmd
        .iter()
        .map(|a| a.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");
    joined.chars().take(200).collect()
}

fn list_cleanup_process_candidates_linux() -> Vec<ProcessCandidate> {
    let my_pid = std::process::id();
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing()
            .with_exe(UpdateKind::Always)
            .with_cmd(UpdateKind::Always),
    );

    let mut out = Vec::new();
    for (pid, proc) in sys.processes() {
        let pid_u32 = pid.as_u32();
        if pid_u32 <= 1 || pid_u32 == my_pid {
            continue;
        }

        let raw_name = proc.name().to_string_lossy();
        if raw_name.starts_with('[') {
            continue;
        }

        let basename = proc
            .exe()
            .and_then(|p| p.file_name().and_then(OsStr::to_str))
            .map(|s| s.to_lowercase())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| raw_name.to_lowercase());

        if denylisted_basename(&basename) {
            continue;
        }

        let preview = {
            let c = proc.cmd();
            if !c.is_empty() {
                cmd_preview(c)
            } else {
                raw_name.to_string()
            }
        };

        out.push(ProcessCandidate {
            pid: pid_u32,
            executable_basename: basename,
            cmd_preview: preview,
        });
    }

    out.sort_by_key(|c| c.pid);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn denylist_hits_system_basenames() {
        assert!(denylisted_basename("systemd"));
        assert!(denylisted_basename("kworker/0:0H"));
        assert!(!denylisted_basename("cursor"));
    }

    #[test]
    fn list_candidates_sorted_and_exclude_low_pids() {
        let v = list_cleanup_process_candidates_linux();
        assert!(v.windows(2).all(|w| w[0].pid <= w[1].pid));
        assert!(v.iter().all(|c| c.pid > 1));
    }
}

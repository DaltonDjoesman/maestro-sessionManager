use std::ffi::OsStr;

use sysinfo::{
    Process, ProcessesToUpdate, ProcessRefreshKind, System, UpdateKind,
};

use super::{PlatformError, ProcessCandidate};

/// Linux adapter (Pop!_OS reference). Uses `sysinfo` for process enumeration.
pub struct LinuxPlatform;

impl LinuxPlatform {
    pub fn new() -> Result<Self, PlatformError> {
        Ok(Self)
    }

    pub fn platform_name(&self) -> &'static str {
        "linux"
    }
}

/// True for normal user-space programs with a readable `/proc/<pid>/exe` (excludes kernel threads).
pub(crate) fn has_resolved_executable(proc: &Process) -> bool {
    let Some(exe) = proc.exe() else {
        return false;
    };
    if exe.as_os_str().is_empty() {
        return false;
    }
    // Kernel threads use names like `migration/0`; real program names from comm never contain `/`.
    if proc.name().to_string_lossy().contains('/') {
        return false;
    }
    true
}

/// Built-in basenames and patterns excluded from assistant noise and similar process lists.
pub(crate) fn denylisted_basename(b: &str) -> bool {
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
        "systemd-oomd",
        "systemd-homed",
        "systemd-userdbd",
        "systemd-userwork",
        "systemd-user-runtime-dir",
        "(sd-pam)",
        "sd-pam",
        "dbus-daemon",
        "dbus-broker",
        "dbus-broker-launch-helper",
        "dbus-launch-helper",
        "polkitd",
        "accounts-daemon",
        "rsyslogd",
        "agetty",
        "login",
        "sshd",
        "cupsd",
        "NetworkManager",
        "wpa_supplicant",
        "iwd",
        "ModemManager",
        "udisksd",
        "udisks",
        "upowerd",
        "colord",
        "power-profiles-daemon",
        "thermald",
        "irqbalance",
        "boltd",
        "packagekitd",
        "fwupd",
        "rtkit-daemon",
        "smartd",
        "haveged",
        "gnome-shell",
        "gnome-shell-calendar-server",
        "gnome-session-binary",
        "gnome-keyring-daemon",
        "gnome-terminal-server",
        "goa-daemon",
        "dconf-service",
        "gcr-ssh-agent",
        "gjs-console",
        "at-spi-bus-launcher",
        "at-spi2-registryd",
        "Xorg",
        "xorg",
        "Xwayland",
        "xwayland",
        "pipewire",
        "pipewire-pulse",
        "wireplumber",
        "pulseaudio",
        "cups-browsed",
        "avahi-daemon",
        "bluetoothd",
        "cron",
        "crond",
        "anacron",
        "kthreadd",
        "khungtaskd",
        "oom_reaper",
        "migration",
        "watchdogd",
        "jbd2",
        "snapd",
        "snap-confine",
        "snapfuse",
        "flatpak-system-helper",
        "flatpak-session-helper",
        "flatpak-portal",
        "xdg-document-portal",
        "xdg-permission-store",
        "xdg-desktop-portal",
        "tracker-extract-3",
        "tracker-miner-fs-3",
        "tracker-miner-fs-2",
        "tracker-writeback-3",
        "kded5",
        "kded6",
        "baloo_file",
        "baloo_file_extractor",
        "gmenudbusmenuproxy",
        "kwalletd5",
        "kwalletd6",
        "ksmserver",
        "containerd",
        "dockerd",
        "docker-proxy",
    ];
    let lower = b.to_lowercase();
    if EXACT.iter().any(|&x| x.eq_ignore_ascii_case(lower.as_str())) {
        return true;
    }
    // Families of long‑running infrastructure daemons (basename is often the helper binary).
    const PREFIX_DENY: &[&str] = &[
        "systemd-",
        "gvfsd",
        "gsd-",
        "evolution-",
        "ibus-",
        "xdg-desktop-portal",
    ];
    if PREFIX_DENY.iter().copied().any(|prefix| lower.starts_with(prefix)) {
        return true;
    }
    if lower.ends_with("crashpad_handler") {
        return true;
    }
    if lower.starts_with("webkit") {
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

/// Post-filtered process list (used by tests; same pipeline historically fed divergence UI).
#[allow(dead_code)]
fn list_user_process_candidates_linux() -> Vec<ProcessCandidate> {
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

        if !has_resolved_executable(proc) {
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
    fn list_candidates_excludes_kernel_style_names() {
        let v = list_user_process_candidates_linux();
        assert!(
            v.iter()
                .all(|c| !c.executable_basename.contains('/')),
            "kernel thread basenames should not appear"
        );
    }

    #[test]
    fn denylist_hits_system_basenames() {
        assert!(denylisted_basename("systemd"));
        assert!(denylisted_basename("kworker/0:0H"));
        assert!(denylisted_basename("packagekitd"));
        assert!(denylisted_basename("gvfsd-metadata"));
        assert!(denylisted_basename("gsd-color"));
        assert!(denylisted_basename("xdg-desktop-portal-hyprland"));
        assert!(denylisted_basename("chrome_crashpad_handler"));
        assert!(denylisted_basename("webkitwebprocess"));
        assert!(!denylisted_basename("cursor"));
    }

    #[test]
    fn list_candidates_sorted_and_exclude_low_pids() {
        let v = list_user_process_candidates_linux();
        assert!(v.windows(2).all(|w| w[0].pid <= w[1].pid));
        assert!(v.iter().all(|c| c.pid > 1));
    }
}

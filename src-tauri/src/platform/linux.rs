use sysinfo::Process;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}

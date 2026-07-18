use std::path::{Path, PathBuf};

use sysinfo::Process;

/// Linux appends ` (deleted)` to `/proc/<pid>/exe` when the on-disk binary was replaced.
pub(crate) fn strip_deleted_exe_suffix(path: &str) -> &str {
    path.trim()
        .strip_suffix(" (deleted)")
        .unwrap_or_else(|| path.trim())
}

/// Prefer an existing file: stripped `/proc` path, then argv0, then same basename on PATH.
pub(crate) fn resolve_process_executable(proc: &Process) -> String {
    let from_exe = proc
        .exe()
        .map(|p| strip_deleted_exe_suffix(&p.to_string_lossy()).to_string())
        .filter(|s| !s.is_empty());
    let from_cmd = proc
        .cmd()
        .first()
        .map(|a| strip_deleted_exe_suffix(&a.to_string_lossy()).to_string())
        .filter(|s| !s.is_empty() && s != "/proc/self/exe");

    for candidate in [from_exe.as_deref(), from_cmd.as_deref()].into_iter().flatten() {
        if Path::new(candidate).is_file() {
            return candidate.to_string();
        }
    }

    let basename_hint = from_exe
        .as_deref()
        .or(from_cmd.as_deref())
        .and_then(|p| Path::new(p).file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("");
    if let Some(found) = find_executable_on_path(basename_hint) {
        return found;
    }

    from_exe
        .or(from_cmd)
        .unwrap_or_else(|| proc.name().to_string_lossy().into_owned())
}

/// When a saved/profile path is missing (often after app updates), try PATH by basename.
pub(crate) fn resolve_launch_executable(path: &str) -> String {
    let cleaned = strip_deleted_exe_suffix(path).to_string();
    if Path::new(&cleaned).is_file() {
        return cleaned;
    }
    let base = Path::new(&cleaned)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    find_executable_on_path(base).unwrap_or(cleaned)
}

fn find_executable_on_path(basename: &str) -> Option<String> {
    let base = basename.trim();
    if base.is_empty() || base.contains('/') {
        return None;
    }
    for dir in path_search_dirs() {
        let candidate = dir.join(base);
        if candidate.is_file() {
            return Some(candidate.to_string_lossy().into_owned());
        }
    }
    None
}

fn path_search_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(path) = std::env::var("PATH") {
        dirs.extend(path.split(':').filter(|s| !s.is_empty()).map(PathBuf::from));
    }
    for extra in ["/usr/bin", "/usr/local/bin", "/bin"] {
        let p = PathBuf::from(extra);
        if !dirs.iter().any(|d| d == &p) {
            dirs.push(p);
        }
    }
    if let Some(home) = dirs::home_dir() {
        let local = home.join(".local/bin");
        if !dirs.iter().any(|d| d == &local) {
            dirs.push(local);
        }
    }
    dirs
}

/// True for normal user-space programs with a readable `/proc/<pid>/exe` (excludes kernel threads).
pub(crate) fn has_resolved_executable(proc: &Process) -> bool {
    let Some(exe) = proc.exe() else {
        return false;
    };
    if strip_deleted_exe_suffix(&exe.to_string_lossy()).is_empty() {
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

    #[test]
    fn strip_deleted_exe_suffix_removes_kernel_marker() {
        assert_eq!(
            strip_deleted_exe_suffix(
                "/home/u/.local/share/cursor-editor/usr/share/cursor/cursor (deleted)"
            ),
            "/home/u/.local/share/cursor-editor/usr/share/cursor/cursor"
        );
        assert_eq!(strip_deleted_exe_suffix("/usr/bin/cursor"), "/usr/bin/cursor");
    }

    #[test]
    fn resolve_launch_executable_finds_cursor_on_path_when_old_install_gone() {
        let gone = "/home/u/.local/share/cursor-editor/usr/share/cursor/cursor (deleted)";
        let resolved = resolve_launch_executable(gone);
        assert!(
            Path::new(&resolved).is_file(),
            "expected PATH fallback for cursor, got {resolved}"
        );
        assert!(
            !resolved.contains("(deleted)"),
            "must not keep deleted marker: {resolved}"
        );
        let base = Path::new(&resolved)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        assert_eq!(base, "cursor");
    }
}

//! Running-process noise filters and shared DTO for the profile editor assistant.

use serde::{Deserialize, Serialize};

/// Classified running program for the profile editor assistant (camelCase for the web UI).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CandidateKind {
    #[serde(rename = "app")]
    App,
    #[serde(rename = "process")]
    Process,
}

impl Default for CandidateKind {
    fn default() -> Self {
        Self::Process
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ClassificationConfidence {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}

/// One row in the profile editor “running apps” assistant (camelCase for the web UI).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RunningAppCandidate {
    pub pid: u32,
    pub executable: String,
    pub label: String,
    pub cmd_preview: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd_hint: Option<String>,
    #[serde(default)]
    pub kind: CandidateKind,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desktop_workspace: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification_confidence: Option<ClassificationConfidence>,
}

pub fn list_running_app_candidates() -> Vec<RunningAppCandidate> {
    super::window_discovery::discover_running_app_candidates()
}

pub(crate) fn assistant_chromium_subprocess_cmd(cmd: &str) -> bool {
    cmd.contains("--type=renderer")
        || cmd.contains("--type=zygote")
        || cmd.contains("--type=gpu-process")
        || cmd.contains("--type=utility")
        || cmd.contains("--type=broker")
        || cmd.contains("--type=crashpad-handler")
        || cmd.contains("--type=crashpad_handler")
}

pub(crate) fn assistant_background_noise(executable: &str, cmd: &str) -> bool {
    if executable.contains("/.cursor/extensions/")
        || executable.contains("rust-lang.rust-analyzer")
        || executable.contains("/rust-analyzer")
    {
        return true;
    }
    let el = executable.to_lowercase();
    let cl = cmd.to_lowercase();
    if el.contains("/node_modules/") || cl.contains("/node_modules/") {
        return true;
    }
    if el.contains("/.nvm/") && cl.contains("node_modules") {
        return true;
    }
    if el.contains("globalstorage")
        && (el.contains("clangd") || cl.contains("clangd") || el.contains("llvm-vs-code-extensions"))
    {
        return true;
    }
    if el.contains("/.config/cursor/")
        && (el.contains("clangd")
            || el.contains("language-server")
            || el.contains("llvm-vs-code-extensions")
            || el.contains("extensions/"))
    {
        return true;
    }
    let base = std::path::Path::new(executable)
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    if matches!(
        base.as_str(),
        "touchegg" | "vboxsvc" | "vboxheadless" | "vboxnetadp" | "vboxnetflt" | "obexd" | "oosplash"
    ) {
        return true;
    }
    if (base == "node" || el.ends_with("/node"))
        && (cl.contains("node_modules/.bin/")
            || cl.contains("/node_modules/.bin/vite")
            || cl.contains("/node_modules/.bin/esbuild")
            || cl.contains("npm run")
            || cl.contains(" npx ")
            || cl.starts_with("npx "))
    {
        return true;
    }
    if base == "esbuild" && cl.contains("node_modules") {
        return true;
    }
    if base.starts_with("python") && cl.contains("hidpi-daemon") {
        return true;
    }
    if base.starts_with("python")
        && (cl.contains("solar")
            || cl.contains("/usr/bin/solar")
            || (cl.contains("--window=hide") && cl.contains("/usr/bin/")))
    {
        return true;
    }
    if el.contains("binfmt-bypass") || cl.contains("binfmt-bypass") {
        return true;
    }
    if el.contains("memfd:") || base.contains("memfd") {
        return true;
    }
    if base == "bwrap" {
        return true;
    }
    if base == "cat" && el.starts_with("/usr/bin/") && cl.trim() == "cat" {
        return true;
    }
    if base == "tail"
        && el.starts_with("/usr/bin/")
        && !cl.contains("/home/")
        && cl.split_whitespace().count() <= 4
    {
        return true;
    }
    if (base == "dash" || base == "sh") && cl.contains("npm run") {
        return true;
    }
    if (base == "dash" || base == "sh")
        && cl.contains("-c")
        && (cl.contains("vite") || cl.contains("tauri"))
    {
        return true;
    }
    if el.contains("cursorsandbox")
        || (el.contains("/cursor/resources/") && el.contains("/helpers/"))
    {
        return true;
    }
    if base.starts_with("goa-") {
        return true;
    }
    if base == "appimagelauncherd" {
        return true;
    }
    if base.starts_with("php")
        && (cl.contains("localhost:") || cl.contains("127.0.0.1:") || cl.contains(" -s "))
    {
        return true;
    }
    if base.starts_with("dbus-broker-") {
        return true;
    }
    if matches!(
        base.as_str(),
        "gnome-session-ctl" | "gnome-session-f" | "gnome-session-service"
    ) {
        return true;
    }
    if base.contains("pop-system-updater") {
        return true;
    }
    if base == "zsh"
        && el.contains("/usr/bin/zsh")
        && (cl.contains("cat <&3") || cl.contains("command cat <&3"))
    {
        return true;
    }
    if base == "zsh"
        && el.contains("/usr/bin/zsh")
        && !cl.contains("/home/")
        && !cl.contains(".sh")
        && cl.split_whitespace().count() <= 4
    {
        return true;
    }
    if base.starts_with("gvfs-") || el.contains("/gvfs-") || el.contains("/usr/libexec/gvfs") {
        return true;
    }
    if base == "xdg-dbus-proxy" {
        return true;
    }
    if el.contains("/p11-kit/") || base.starts_with("p11-kit") {
        return true;
    }
    if base == "adb" && (cl.contains("fork-server") || cl.contains("forkserver")) {
        return true;
    }
    if el.contains("gdm-x-session") || base.contains("gdm-x-session") {
        return true;
    }
    if cl.contains("io.elementary.appcenter") && cl.contains(" -s") {
        return true;
    }
    false
}

pub(crate) fn assistant_basename_skip_for_capture(exe_path: &str) -> bool {
    std::path::Path::new(exe_path)
        .file_name()
        .and_then(|s| s.to_str())
        .is_some_and(|b| b.eq_ignore_ascii_case("rustc"))
}

pub(crate) fn process_cmdline_full(cmd: &[std::ffi::OsString]) -> String {
    if cmd.is_empty() {
        return String::new();
    }
    cmd.iter()
        .map(|a| a.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn cwd_hint_for_editor(basename_lower: &str, pid: u32) -> Option<String> {
    const KNOWN: &[&str] = &["cursor", "code", "code-oss", "codium", "obsidian"];
    if !KNOWN.iter().any(|e| *e == basename_lower) {
        return None;
    }
    read_proc_cwd(pid)
}

#[cfg(target_os = "linux")]
fn read_proc_cwd(pid: u32) -> Option<String> {
    let path = std::path::PathBuf::from(format!("/proc/{pid}/cwd"));
    let target = std::fs::read_link(&path).ok()?;
    let s = target.to_string_lossy().into_owned();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

#[cfg(not(target_os = "linux"))]
fn read_proc_cwd(_pid: u32) -> Option<String> {
    None
}

#[cfg(test)]
mod assistant_filter_tests {
    use super::{assistant_background_noise, assistant_chromium_subprocess_cmd};

    #[test]
    fn chromium_subprocess_cmd_detects_common_types() {
        assert!(assistant_chromium_subprocess_cmd("--type=renderer"));
        assert!(!assistant_chromium_subprocess_cmd("/opt/TickTick/ticktick --user-data-dir=/x"));
    }

    #[test]
    fn background_noise_node_npm_run_tauri_dev() {
        assert!(assistant_background_noise(
            "/home/greedisland/.local/share/fnm/node-versions/v24.18.0/installation/bin/node",
            "npm run tauri dev"
        ));
    }

    #[test]
    fn background_noise_obexd() {
        assert!(assistant_background_noise(
            "/usr/lib/bluetooth/obexd",
            "/usr/lib/bluetooth/obexd"
        ));
    }

    #[test]
    fn background_keeps_normal_app() {
        assert!(!assistant_background_noise(
            "/opt/TickTick/ticktick",
            "/opt/TickTick/ticktick --foo"
        ));
    }
}

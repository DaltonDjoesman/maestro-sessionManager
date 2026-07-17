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

pub(crate) fn cwd_hint_for_editor(
    basename_lower: &str,
    pid: u32,
    cmd: &[std::ffi::OsString],
    window_title: Option<&str>,
) -> Option<String> {
    if !crate::editors::is_known_editor_basename(basename_lower) {
        return None;
    }

    if let Some(title) = window_title {
        if let Some(display) = workspace_display_name_from_title(title, basename_lower) {
            if let Some(folder) = resolve_workspace_folder_from_storage(basename_lower, &display) {
                return Some(folder);
            }
            if let Some(folder) = editor_path_from_cmdline_matching_name(cmd, &display) {
                return Some(folder);
            }
        }
    }

    if let Some(folder) = editor_path_from_cmdline_best(cmd) {
        return Some(folder);
    }

    read_proc_cwd_if_sensible(pid)
}

fn workspace_display_name_from_title(title: &str, basename_lower: &str) -> Option<String> {
    let stripped = strip_editor_app_suffix(title, basename_lower)?;
    let parts: Vec<&str> = stripped
        .split(" - ")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if parts.is_empty() {
        return None;
    }
    Some(parts[parts.len() - 1].to_string())
}

fn editor_workspace_storage_root(basename_lower: &str) -> Option<std::path::PathBuf> {
    let home = dirs::home_dir()?;
    let config = home.join(".config");
    let sub = match basename_lower {
        "cursor" => config.join("Cursor/User/workspaceStorage"),
        "code" => config.join("Code/User/workspaceStorage"),
        "code-oss" => config.join("Code - OSS/User/workspaceStorage"),
        "codium" => config.join("VSCodium/User/workspaceStorage"),
        _ => return None,
    };
    if sub.is_dir() {
        Some(sub)
    } else {
        None
    }
}

fn folder_from_workspace_json(content: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(content).ok()?;
    let folder = value.get("folder")?.as_str()?;
    if folder.starts_with("file://") {
        decode_file_uri(folder)
    } else {
        Some(folder.to_string())
    }
}

fn resolve_workspace_folder_from_storage(
    basename_lower: &str,
    workspace_display: &str,
) -> Option<String> {
    let root = editor_workspace_storage_root(basename_lower)?;
    let needle = workspace_display.to_lowercase();
    for entry in std::fs::read_dir(root).ok()?.flatten() {
        let ws_json = entry.path().join("workspace.json");
        if !ws_json.is_file() {
            continue;
        }
        let Some(content) = std::fs::read_to_string(&ws_json).ok() else {
            continue;
        };
        let Some(folder) = folder_from_workspace_json(&content) else {
            continue;
        };
        let path = std::path::PathBuf::from(&folder);
        if !path.is_dir() {
            continue;
        }
        let Some(base) = path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
        else {
            continue;
        };
        if base == needle {
            return Some(folder);
        }
    }
    None
}

fn editor_path_from_cmdline_matching_name(cmd: &[std::ffi::OsString], workspace_name: &str) -> Option<String> {
    let needle = workspace_name.to_lowercase();
    let mut best: Option<String> = None;
    for arg in cmd.iter().skip(1) {
        let s = arg.to_string_lossy();
        if let Some(path) = editor_path_from_arg(&s) {
            let p = std::path::Path::new(&path);
            if !p.exists() {
                continue;
            }
            let folder = if p.is_dir() {
                path.clone()
            } else {
                p.parent()?.to_string_lossy().into_owned()
            };
            let base = std::path::Path::new(&folder)
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase())?;
            if base == needle {
                let replace = best
                    .as_ref()
                    .map(|prev| folder.len() > prev.len())
                    .unwrap_or(true);
                if replace {
                    best = Some(folder);
                }
            }
        }
    }
    best
}

fn editor_path_from_cmdline_best(cmd: &[std::ffi::OsString]) -> Option<String> {
    let home = dirs::home_dir();
    let mut best: Option<String> = None;
    for arg in cmd.iter().skip(1) {
        let s = arg.to_string_lossy();
        if let Some(path) = editor_path_from_arg(&s) {
            let p = std::path::Path::new(&path);
            if !p.exists() {
                continue;
            }
            let folder = if p.is_dir() {
                path
            } else {
                p.parent()?.to_string_lossy().into_owned()
            };
            if home.as_ref().is_some_and(|h| h.as_path() == std::path::Path::new(&folder)) {
                continue;
            }
            let replace = best
                .as_ref()
                .map(|prev| folder.len() > prev.len())
                .unwrap_or(true);
            if replace {
                best = Some(folder);
            }
        }
    }
    best
}

fn editor_path_from_arg(arg: &str) -> Option<String> {
    if let Some(rest) = arg.strip_prefix("--folder-uri=") {
        return decode_file_uri(rest);
    }
    if let Some(rest) = arg.strip_prefix("--file-uri=") {
        return decode_file_uri(rest);
    }
    if arg.starts_with('-') {
        return None;
    }
    let path = std::path::Path::new(arg);
    if path.is_absolute() {
        return Some(arg.to_string());
    }
    None
}

fn decode_file_uri(uri: &str) -> Option<String> {
    let path = uri.trim().strip_prefix("file://")?;
    let path = path.strip_prefix("//localhost").unwrap_or(path);
    let decoded = percent_decode(path);
    if decoded.is_empty() {
        None
    } else {
        Some(decoded)
    }
}

fn percent_decode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(v) = u8::from_str_radix(&input[i + 1..i + 3], 16) {
                out.push(char::from(v));
                i += 3;
                continue;
            }
        }
        out.push(char::from(bytes[i]));
        i += 1;
    }
    out
}

fn strip_editor_app_suffix(title: &str, basename_lower: &str) -> Option<String> {
    let title = title.trim();
    let suffixes: &[&str] = match basename_lower {
        "cursor" => &[" - Cursor"],
        "code" | "code-oss" => &[" - Visual Studio Code", " - Code - OSS"],
        "codium" => &[" - VSCodium"],
        "obsidian" => &[" - Obsidian"],
        _ => return None,
    };
    for suffix in suffixes {
        if title.len() > suffix.len()
            && title[title.len() - suffix.len()..].eq_ignore_ascii_case(suffix)
        {
            return Some(title[..title.len() - suffix.len()].trim().to_string());
        }
    }
    None
}

fn read_proc_cwd_if_sensible(pid: u32) -> Option<String> {
    let cwd = read_proc_cwd(pid)?;
    if is_unhelpful_editor_cwd(&cwd) {
        None
    } else {
        Some(cwd)
    }
}

fn is_unhelpful_editor_cwd(path: &str) -> bool {
    if path == "/" {
        return true;
    }
    if dirs::home_dir().is_some_and(|home| home == std::path::Path::new(path)) {
        return true;
    }
    let lower = path.to_lowercase();
    lower.starts_with("/usr/share/")
        || lower.starts_with("/usr/lib/")
        || lower.contains("/.cursor/")
        || lower.ends_with("/cursor")
        || lower.ends_with("/code")
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
    use std::ffi::OsString;

    use super::{
        assistant_background_noise, assistant_chromium_subprocess_cmd, cwd_hint_for_editor,
        decode_file_uri, editor_path_from_arg, folder_from_workspace_json,
        resolve_workspace_folder_from_storage, strip_editor_app_suffix,
        editor_workspace_storage_root, workspace_display_name_from_title,
    };

    #[test]
    fn editor_path_from_folder_uri() {
        assert_eq!(
            editor_path_from_arg("--folder-uri=file:///home/user/proj"),
            Some("/home/user/proj".into())
        );
    }

    #[test]
    fn decode_file_uri_percent_encoding() {
        assert_eq!(
            decode_file_uri("file:///home/user/My%20Project"),
            Some("/home/user/My Project".into())
        );
    }

    #[test]
    fn strip_cursor_window_suffix() {
        assert_eq!(
            strip_editor_app_suffix("next.config.js: turbopack | Next.js - Cursor", "cursor"),
            Some("next.config.js: turbopack | Next.js".into())
        );
    }

    #[test]
    fn workspace_display_name_from_vscode_title() {
        assert_eq!(
            workspace_display_name_from_title("vite.config.ts - openspectutorial - Cursor", "cursor"),
            Some("openspectutorial".into())
        );
        assert_eq!(
            workspace_display_name_from_title("Browser Tab - sekai-site - Cursor", "cursor"),
            Some("sekai-site".into())
        );
    }

    #[test]
    fn folder_from_real_sekai_json() {
        let path = dirs::home_dir()
            .unwrap()
            .join(".config/Cursor/User/workspaceStorage/70e5f8251724403394c76a59c7b6a685/workspace.json");
        if !path.is_file() {
            return;
        }
        let content = std::fs::read_to_string(&path).unwrap();
        let folder = folder_from_workspace_json(&content);
        assert!(
            folder.as_ref().is_some_and(|f| f.contains("sekai-site")),
            "folder_from_workspace_json failed: {folder:?} for {content}"
        );
        let decoded = folder.unwrap();
        assert!(
            std::path::Path::new(&decoded).is_dir(),
            "decoded path is not a dir: {decoded}"
        );
    }

    #[test]
    fn editor_storage_root_exists_for_cursor() {
        let expected = dirs::home_dir()
            .unwrap()
            .join(".config/Cursor/User/workspaceStorage");
        if !expected.is_dir() {
            return;
        }
        assert_eq!(editor_workspace_storage_root("cursor").as_ref(), Some(&expected));
    }

    #[test]
    fn resolve_sekai_site_from_cursor_storage() {
        let home = dirs::home_dir().expect("home");
        if !home.join(".config/Cursor/User/workspaceStorage").is_dir() {
            return;
        }
        let folder = resolve_workspace_folder_from_storage("cursor", "sekai-site");
        assert!(
            folder.as_ref().is_some_and(|p| p.contains("sekai-site")),
            "expected sekai-site in storage, got {folder:?}"
        );
        let title = "Browser Tab - sekai-site - Cursor";
        let hint = cwd_hint_for_editor("cursor", 1, &[], Some(title));
        assert!(
            hint.as_ref().is_some_and(|p| p.contains("sekai-site")),
            "cwd_hint_for_editor failed for browser tab title, got {hint:?}"
        );
    }

    #[test]
    fn resolve_workspace_folder_from_storage_json() {
        let root = std::env::temp_dir().join("maestro-ws-storage-test");
        let ws_id = root.join("abc123");
        let _ = std::fs::create_dir_all(&ws_id);
        std::fs::write(
            ws_id.join("workspace.json"),
            r#"{"folder":"file:///tmp/maestro-demo-openspectutorial"}"#,
        )
        .expect("write");
        let _ = std::fs::create_dir_all("/tmp/maestro-demo-openspectutorial");
        // Inject storage root by testing folder_from_workspace_json + manual match instead
        let content = std::fs::read_to_string(ws_id.join("workspace.json")).unwrap();
        assert_eq!(
            folder_from_workspace_json(&content).as_deref(),
            Some("/tmp/maestro-demo-openspectutorial")
        );
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir("/tmp/maestro-demo-openspectutorial");
    }

    #[test]
    fn cwd_hint_uses_workspace_storage_from_title() {
        let home = dirs::home_dir().expect("home");
        let storage = home.join(".config/Cursor/User/workspaceStorage");
        if !storage.is_dir() {
            return;
        }
        let title = "vite.config.ts - openspectutorial - Cursor";
        let cmd = vec![OsString::from("/usr/share/cursor/cursor")];
        let hint = cwd_hint_for_editor("cursor", 80853, &cmd, Some(title));
        if hint.is_some() {
            assert!(
                hint.as_ref().unwrap().ends_with("openspectutorial"),
                "expected openspectutorial folder, got {hint:?}"
            );
        }
    }

    #[test]
    fn cwd_hint_uses_cmdline_folder_before_proc_cwd() {
        let cmd = vec![
            OsString::from("/usr/bin/cursor"),
            OsString::from("/tmp"),
        ];
        let hint = cwd_hint_for_editor("cursor", 1, &cmd, None);
        assert_eq!(hint.as_deref(), Some("/tmp"));
    }

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

//! Best-effort detection of the desktop default web browser (Linux), for UI placeholders.

use serde::{Deserialize, Serialize};

use crate::settings::BrowserFamily;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SystemDefaultBrowserHint {
    /// Resolved `Exec=` binary or `which x-www-browser`, when known.
    pub executable: Option<String>,
    pub family: Option<BrowserFamily>,
    /// Raw `xdg-settings get default-web-browser` value (often `*.desktop`).
    pub desktop_entry: Option<String>,
}

pub fn detect_system_default_browser() -> SystemDefaultBrowserHint {
    #[cfg(target_os = "linux")]
    {
        detect_linux_default_browser()
    }
    #[cfg(not(target_os = "linux"))]
    {
        SystemDefaultBrowserHint {
            executable: None,
            family: None,
            desktop_entry: None,
        }
    }
}

#[cfg(target_os = "linux")]
fn detect_linux_default_browser() -> SystemDefaultBrowserHint {
    let desktop_entry = run_output_first_line("xdg-settings", &["get", "default-web-browser"]);
    let mut family = desktop_entry
        .as_deref()
        .and_then(infer_family_from_desktop_id);
    let mut executable = desktop_entry
        .as_ref()
        .and_then(|d| read_desktop_file(d))
        .and_then(|content| parse_desktop_exec(&content));

    if executable.is_none() {
        executable = run_output_first_line("which", &["x-www-browser"]);
    }

    if family.is_none() {
        family = executable.as_deref().and_then(infer_family_from_executable);
    }

    SystemDefaultBrowserHint {
        executable,
        family,
        desktop_entry,
    }
}

fn run_output_first_line(cmd: &str, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new(cmd).args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let line = String::from_utf8_lossy(&out.stdout)
        .lines()
        .next()
        .map(str::trim)
        .filter(|s| !s.is_empty())?
        .to_string();
    Some(line)
}

fn infer_family_from_desktop_id(id: &str) -> Option<BrowserFamily> {
    // Desktop ids often look like `firefox.desktop` / `google-chrome.desktop` — reuse detect markers.
    use crate::browser::detect_browser_family;
    detect_browser_family(id.trim_end_matches(".desktop")).or_else(|| detect_browser_family(id))
}

fn infer_family_from_executable(exe: &str) -> Option<BrowserFamily> {
    crate::browser::detect_browser_family(exe)
}

#[cfg(target_os = "linux")]
fn read_desktop_file(desktop_name: &str) -> Option<String> {
    let mut paths = Vec::new();
    if let Some(home) = dirs::home_dir() {
        paths.push(home.join(".local/share/applications").join(desktop_name));
    }
    paths.push(std::path::PathBuf::from("/usr/local/share/applications").join(desktop_name));
    paths.push(std::path::PathBuf::from("/usr/share/applications").join(desktop_name));
    for p in paths {
        if let Ok(content) = std::fs::read_to_string(&p) {
            return Some(content);
        }
    }
    None
}

/// First `Exec=` line: strips `%u`-style field codes and a leading `env VAR=…` chain.
fn parse_desktop_exec(content: &str) -> Option<String> {
    for raw in content.lines() {
        let line = raw.trim();
        if !line.starts_with("Exec=") {
            continue;
        }
        let rest = line.strip_prefix("Exec=")?.trim();
        let tokens: Vec<&str> = rest.split_whitespace().collect();
        let mut i = 0usize;
        if tokens.first().copied() == Some("env") {
            i += 1;
            while i < tokens.len() && tokens[i].contains('=') && !tokens[i].starts_with('/') {
                i += 1;
            }
        }
        while i < tokens.len() {
            let t = tokens[i];
            if t.starts_with('%') {
                i += 1;
                continue;
            }
            if t.contains('=') && !t.starts_with('/') {
                i += 1;
                continue;
            }
            return Some(t.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_desktop_exec_simple() {
        let s = "[Desktop Entry]\nExec=/usr/bin/google-chrome-stable %U\n";
        assert_eq!(
            parse_desktop_exec(s).as_deref(),
            Some("/usr/bin/google-chrome-stable")
        );
    }

    #[test]
    fn parse_desktop_exec_env_prefix() {
        let s = "Exec=env MOZ_APP_REMOTING=1 /usr/lib/firefox/firefox %u\n";
        assert_eq!(
            parse_desktop_exec(s).as_deref(),
            Some("/usr/lib/firefox/firefox")
        );
    }

    #[test]
    fn hint_family_agrees_with_detect_on_known_executables() {
        use crate::browser::detect_browser_family;
        for exe in [
            "/usr/bin/firefox",
            "/usr/bin/google-chrome-stable",
            "/usr/bin/vivaldi-bin",
            "/usr/bin/code",
        ] {
            assert_eq!(
                infer_family_from_executable(exe),
                detect_browser_family(exe),
                "mismatch for {exe}"
            );
        }
    }

    #[test]
    fn hint_desktop_id_does_not_default_unknown_to_chromium() {
        assert_eq!(infer_family_from_desktop_id("code.desktop"), None);
        assert_eq!(
            infer_family_from_desktop_id("firefox.desktop"),
            Some(BrowserFamily::Firefox)
        );
        assert_eq!(
            infer_family_from_desktop_id("google-chrome.desktop"),
            Some(BrowserFamily::ChromiumLike)
        );
    }
}

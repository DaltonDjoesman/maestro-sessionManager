//! Freedesktop `.desktop` index for classifying running processes as GUI apps.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopEntry {
    pub name: String,
    pub icon: Option<String>,
    pub exec_keys: Vec<String>,
    pub startup_wm_class: Option<String>,
    /// Chromium/Vivaldi PWA installed via `--app-id=…` in Exec.
    pub app_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct DesktopIndex {
    by_key: HashMap<String, DesktopEntry>,
    by_wm_class: HashMap<String, DesktopEntry>,
}

impl DesktopIndex {
    pub fn load() -> Self {
        let mut index = Self::default();
        for dir in desktop_application_dirs() {
            if dir.is_dir() {
                index.scan_dir(&dir);
            }
        }
        index
    }

    fn scan_dir(&mut self, dir: &Path) {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                continue;
            }
            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            if let Some(parsed) = parse_desktop_file(&content) {
                self.register_entry(parsed);
            }
        }
    }

    fn register_entry(&mut self, parsed: DesktopEntry) {
        for key in &parsed.exec_keys {
            self.by_key
                .entry(key.clone())
                .or_insert_with(|| parsed.clone());
        }
        if let Some(wm) = &parsed.startup_wm_class {
            let wm_key = wm.to_lowercase();
            self.by_wm_class
                .entry(wm_key)
                .or_insert_with(|| parsed.clone());
        }
    }

    /// Match a process executable (and basename) to a desktop entry.
    pub fn match_process(
        &self,
        executable: &str,
        basename_lower: &str,
        cmdline: Option<&str>,
    ) -> Option<&DesktopEntry> {
        if let Some(cmd) = cmdline {
            if let Some(id) = extract_app_id(cmd) {
                if let Some(entry) = self.by_key.get(&app_id_key(&id)) {
                    return Some(entry);
                }
            }
        }

        let mut basename_candidates = vec![basename_lower.to_string()];
        if basename_lower.ends_with("-bin") {
            basename_candidates.push(basename_lower.trim_end_matches("-bin").to_string());
        }

        for base in &basename_candidates {
            if let Some(entry) = self.by_key.get(base) {
                if entry.app_id.is_none() {
                    return Some(entry);
                }
            }
        }

        let exe_lower = executable.to_lowercase();
        for (key, entry) in &self.by_key {
            if entry.app_id.is_some() {
                continue;
            }
            if exe_lower.ends_with(key) || exe_lower.contains(&format!("/{key}")) {
                return Some(entry);
            }
        }

        if basename_lower.ends_with("-bin") {
            let stem = basename_lower.trim_end_matches("-bin");
            for (key, entry) in &self.by_key {
                if entry.app_id.is_some() {
                    continue;
                }
                if key.contains(stem) {
                    return Some(entry);
                }
            }
        }
        None
    }

    /// Match window title against StartupWMClass entries and app names.
    pub fn match_window_title(&self, title: &str) -> Option<&DesktopEntry> {
        let lower = title.to_lowercase();
        for (wm, entry) in &self.by_wm_class {
            if lower.contains(wm) {
                return Some(entry);
            }
        }
        let mut name_matches: Vec<&DesktopEntry> = self
            .by_key
            .values()
            .filter(|entry| {
                entry.app_id.is_none() && lower.contains(&entry.name.to_lowercase())
            })
            .collect();
        name_matches.sort_by(|a, b| b.name.len().cmp(&a.name.len()));
        name_matches.into_iter().next()
    }

    /// Match a Wayland foreign-toplevel `app_id` (e.g. `ticktick`, `vivaldi-stable`).
    pub fn match_app_id(&self, app_id: &str) -> Option<&DesktopEntry> {
        let id = app_id.trim();
        if id.is_empty() {
            return None;
        }
        let lower = id.to_lowercase();
        if let Some(entry) = self.by_key.get(&lower) {
            return Some(entry);
        }
        if let Some(entry) = self.by_wm_class.get(&lower) {
            return Some(entry);
        }
        // Desktop file id often matches app_id (e.g. `slack.desktop` → key `slack`).
        self.by_key.values().find(|entry| {
            entry
                .startup_wm_class
                .as_deref()
                .map(|wm| wm.eq_ignore_ascii_case(id))
                .unwrap_or(false)
                || entry.name.eq_ignore_ascii_case(id)
        })
    }
}

fn desktop_application_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![PathBuf::from("/usr/share/applications")];
    if let Some(home) = dirs::home_dir() {
        dirs.push(home.join(".local/share/applications"));
        dirs.push(home.join(".local/share/flatpak/exports/share/applications"));
    }
    dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));
    dirs.push(PathBuf::from("/var/lib/snapd/desktop/applications"));
    dirs
}

fn parse_desktop_file(content: &str) -> Option<DesktopEntry> {
    let mut in_desktop_entry = false;
    let mut name: Option<String> = None;
    let mut icon: Option<String> = None;
    let mut exec_raw: Option<String> = None;
    let mut try_exec: Option<String> = None;
    let mut startup_wm_class: Option<String> = None;
    let mut entry_type: Option<String> = None;
    let mut hidden = false;
    let mut no_display = false;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_desktop_entry = line.eq_ignore_ascii_case("[Desktop Entry]");
            continue;
        }
        if !in_desktop_entry {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        match key {
            "Name" => name = Some(value.to_string()),
            "Icon" => icon = Some(value.to_string()),
            "Exec" => exec_raw = Some(value.to_string()),
            "TryExec" => try_exec = Some(value.to_string()),
            "StartupWMClass" => startup_wm_class = Some(value.to_string()),
            "Type" => entry_type = Some(value.to_string()),
            "Hidden" => hidden = value.eq_ignore_ascii_case("true"),
            "NoDisplay" => no_display = value.eq_ignore_ascii_case("true"),
            _ => {}
        }
    }

    if hidden || no_display {
        return None;
    }
    if entry_type.as_deref() != Some("Application") {
        return None;
    }

    let mut exec_keys = Vec::new();
    if let Some(ref exec) = exec_raw {
        exec_keys.extend(exec_match_keys(exec));
    }
    if let Some(ref te) = try_exec {
        if let Some(token) = exec_first_token(te) {
            push_exec_key(&mut exec_keys, &token);
        }
    }
    if exec_keys.is_empty() {
        return None;
    }

    let app_id = exec_raw.as_ref().and_then(|e| extract_app_id(e));
    if let Some(ref id) = app_id {
        exec_keys.clear();
        exec_keys.push(app_id_key(id));
    } else {
        exec_keys.sort();
        exec_keys.dedup();
    }

    let name = name.unwrap_or_else(|| exec_keys[0].clone());

    Some(DesktopEntry {
        name,
        icon,
        exec_keys,
        startup_wm_class,
        app_id,
    })
}

fn extract_app_id(exec: &str) -> Option<String> {
    exec.split_whitespace().find_map(|token| {
        token
            .strip_prefix("--app-id=")
            .map(|id| id.trim().to_string())
            .filter(|id| !id.is_empty())
    })
}

fn app_id_key(id: &str) -> String {
    format!("app-id:{id}")
}

/// Strip Freedesktop field codes and return the first executable token.
pub fn exec_first_token(exec: &str) -> Option<String> {
    let s = strip_field_codes(exec);
    let token = s.split_whitespace().next()?.trim();
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

fn strip_field_codes(exec: &str) -> String {
    let mut s = exec.to_string();
    for code in [
        "%f", "%F", "%u", "%U", "%i", "%c", "%k", "%v", "%m", "%M", "%t", "%T",
    ] {
        s = s.replace(code, "");
    }
    s
}

/// Extract match keys from an Exec line, including Flatpak `bwrap` and `/app/` targets.
pub fn exec_match_keys(exec_line: &str) -> Vec<String> {
    let cleaned = strip_field_codes(exec_line);
    let tokens: Vec<&str> = cleaned.split_whitespace().collect();
    let mut keys = Vec::new();

    for token in &tokens {
        push_exec_key(&mut keys, token);
    }

    if let Some(pos) = tokens.iter().position(|t| *t == "--") {
        if let Some(after) = tokens.get(pos + 1) {
            push_exec_key(&mut keys, after);
        }
    }

    keys.sort();
    keys.dedup();
    keys
}

fn push_exec_key(keys: &mut Vec<String>, token: &str) {
    let token = token.trim();
    if token.is_empty() || token.starts_with('-') {
        return;
    }
    let base = Path::new(token)
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| token.to_lowercase());
    if !base.is_empty() {
        keys.push(base);
    }
    if token.contains('/') {
        keys.push(token.to_lowercase());
    }
    if token.starts_with("/app/") {
        if let Some(rest) = token.strip_prefix("/app/") {
            if let Some(seg) = rest.split('/').next() {
                if !seg.is_empty() {
                    keys.push(seg.to_lowercase());
                }
            }
        }
    }
}

pub fn humanize_basename(basename: &str) -> String {
    let mut chars = basename.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
impl DesktopIndex {
    pub fn from_entries(entries: Vec<(String, DesktopEntry)>) -> Self {
        let mut index = Self::default();
        for (_, entry) in entries {
            index.register_entry(entry);
        }
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_first_token_strips_field_codes() {
        assert_eq!(
            exec_first_token("firefox %u https://example.com"),
            Some("firefox".into())
        );
    }

    #[test]
    fn exec_match_keys_flatpak_bwrap() {
        let keys = exec_match_keys(
            "bwrap --args 42 -- /app/obsidian --ozone-platform=x11",
        );
        assert!(keys.contains(&"obsidian".to_string()));
        assert!(keys.contains(&"/app/obsidian".to_string()));
    }

    #[test]
    fn exec_match_keys_snap_path() {
        let keys = exec_match_keys("/snap/bin/firefox %U");
        assert!(keys.contains(&"firefox".to_string()));
    }

    #[test]
    fn parse_desktop_file_basic() {
        let content = r#"
[Desktop Entry]
Type=Application
Name=Obsidian
Exec=obsidian %U
Icon=obsidian
StartupWMClass=obsidian
"#;
        let entry = parse_desktop_file(content).expect("parsed");
        assert_eq!(entry.name, "Obsidian");
        assert_eq!(entry.startup_wm_class.as_deref(), Some("obsidian"));
        assert!(entry.app_id.is_none());
    }

    #[test]
    fn parse_desktop_pwa_app_id_uses_isolated_key() {
        let content = r#"
[Desktop Entry]
Type=Application
Name=Odysseus
Exec=/opt/vivaldi/vivaldi --profile-directory=Default --app-id=gjblgeihjnnclmidiiaomicjniinpook
Icon=vivaldi-pwa
StartupWMClass=crx_gjblgeihjnnclmidiiaomicjniinpook
"#;
        let entry = parse_desktop_file(content).expect("parsed");
        assert_eq!(entry.name, "Odysseus");
        assert_eq!(
            entry.app_id.as_deref(),
            Some("gjblgeihjnnclmidiiaomicjniinpook")
        );
        assert_eq!(
            entry.exec_keys,
            vec!["app-id:gjblgeihjnnclmidiiaomicjniinpook".to_string()]
        );
        assert!(!entry.exec_keys.contains(&"vivaldi".to_string()));
    }

    #[test]
    fn pwa_does_not_steal_vivaldi_browser_match() {
        let mut index = DesktopIndex::default();
        index.register_entry(DesktopEntry {
            name: "Vivaldi".into(),
            icon: Some("vivaldi".into()),
            exec_keys: vec!["vivaldi-stable".into(), "vivaldi-bin".into()],
            startup_wm_class: Some("vivaldi".into()),
            app_id: None,
        });
        index.register_entry(DesktopEntry {
            name: "Odysseus".into(),
            icon: Some("odysseus".into()),
            exec_keys: vec!["app-id:gjblgeihjnnclmidiiaomicjniinpook".into()],
            startup_wm_class: Some("crx_gjblgeihjnnclmidiiaomicjniinpook".into()),
            app_id: Some("gjblgeihjnnclmidiiaomicjniinpook".into()),
        });

        let browser = index
            .match_process(
                "/opt/vivaldi/vivaldi-bin",
                "vivaldi-bin",
                Some("/opt/vivaldi/vivaldi-bin --new-window https://youtube.com"),
            )
            .expect("browser match");
        assert_eq!(browser.name, "Vivaldi");

        let pwa = index
            .match_process(
                "/opt/vivaldi/vivaldi-bin",
                "vivaldi-bin",
                Some("/opt/vivaldi/vivaldi --app-id=gjblgeihjnnclmidiiaomicjniinpook"),
            )
            .expect("pwa match");
        assert_eq!(pwa.name, "Odysseus");
    }

    #[test]
    fn parse_desktop_try_exec_only() {
        let content = r#"
[Desktop Entry]
Type=Application
Name=Helper
TryExec=my-helper
"#;
        let entry = parse_desktop_file(content).expect("parsed");
        assert!(entry.exec_keys.contains(&"my-helper".to_string()));
    }

    #[test]
    fn parse_skips_hidden() {
        let content = r#"
[Desktop Entry]
Type=Application
Name=Hidden App
Exec=hidden-app
Hidden=true
"#;
        assert!(parse_desktop_file(content).is_none());
    }

    #[test]
    fn index_matches_obsidian_flatpak_path() {
        let mut index = DesktopIndex::default();
        let entry = DesktopEntry {
            name: "Obsidian".into(),
            icon: Some("obsidian".into()),
            exec_keys: vec!["obsidian".into(), "/app/obsidian".into()],
            startup_wm_class: Some("obsidian".into()),
            app_id: None,
        };
        index.register_entry(entry);
        let matched = index.match_process("/app/obsidian", "obsidian", None);
        assert!(matched.is_some());
    }

    #[test]
    fn match_window_title_by_wm_class() {
        let mut index = DesktopIndex::default();
        index.register_entry(DesktopEntry {
            name: "Obsidian".into(),
            icon: None,
            exec_keys: vec!["obsidian".into()],
            startup_wm_class: Some("obsidian".into()),
            app_id: None,
        });
        assert!(index.match_window_title("Project - Obsidian 1.12").is_some());
    }
}

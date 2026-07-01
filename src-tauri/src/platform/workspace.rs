//! Best-effort desktop workspace read (X11 via wmctrl). Does not move windows.

use std::collections::HashMap;
use std::process::Command;

/// `XDG_SESSION_TYPE` value, or `"unknown"`.
pub fn session_type() -> String {
    std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "unknown".into())
}

/// One mapped top-level window from `wmctrl -l -p`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowRecord {
    pub pid: u32,
    pub desktop: u32,
    pub title: String,
}

/// PID → workspace index (0-based) when resolvable, plus raw window list for title matching.
#[derive(Debug, Default)]
pub struct WorkspaceIndex {
    pid_to_desktop: HashMap<u32, u32>,
    windows: Vec<WindowRecord>,
}

impl WorkspaceIndex {
    pub fn load() -> Self {
        let mut index = Self::default();
        let st = session_type();
        // X11 and hybrid sessions (some apps use --ozone-platform=x11 on Wayland hosts).
        if st == "x11" || st == "tty" {
            index.load_wmctrl();
        }
        // Wayland / Cosmic: no stable cross-compositor workspace API yet.
        index
    }

    /// Uses `wmctrl -l -p` subprocess (avoids pulling in x11rb for MVP phase 2).
    fn load_wmctrl(&mut self) {
        let output = match Command::new("wmctrl").args(["-l", "-p"]).output() {
            Ok(o) if o.status.success() => o,
            _ => return,
        };
        let text = String::from_utf8_lossy(&output.stdout);
        parse_wmctrl_lp(&text, &mut self.windows, &mut self.pid_to_desktop);
    }

    pub fn windows(&self) -> &[WindowRecord] {
        &self.windows
    }

    pub fn has_window_for_pid(&self, pid: u32) -> bool {
        if pid <= 10 {
            return false;
        }
        self.pid_to_desktop.contains_key(&pid)
    }

    /// Direct map, then walk `/proc` parent chain (window PID often differs from child workers).
    pub fn workspace_for_pid(&self, pid: u32) -> Option<u32> {
        self.resolve_workspace_for_pid(pid, 12)
    }

    fn resolve_workspace_for_pid(&self, pid: u32, max_depth: u32) -> Option<u32> {
        let mut current = pid;
        for _ in 0..max_depth {
            if current <= 10 {
                break;
            }
            if let Some(ws) = self.pid_to_desktop.get(&current) {
                return Some(*ws);
            }
            let Some(ppid) = read_ppid(current) else {
                break;
            };
            if ppid == current {
                break;
            }
            current = ppid;
        }
        None
    }

    /// Match wmctrl title when PID is bogus (0/2) or points at a worker process.
    pub fn workspace_for_title_hint(&self, hint: &str) -> Option<u32> {
        let hint = hint.trim();
        if hint.is_empty() {
            return None;
        }
        let lower = hint.to_lowercase();
        self.windows
            .iter()
            .filter(|w| w.title.to_lowercase().contains(&lower))
            .map(|w| w.desktop)
            .min()
    }

    /// Match executable basename against window titles (e.g. `ticktick` → "TickTick").
    pub fn workspace_for_executable(&self, executable: &str, display_name: &str) -> Option<u32> {
        if let Some(ws) = self.workspace_for_title_hint(display_name) {
            return Some(ws);
        }
        let base = std::path::Path::new(executable)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if base.len() >= 3 {
            if let Some(ws) = self.workspace_for_title_hint(base) {
                return Some(ws);
            }
        }
        None
    }
}

/// Parse `wmctrl -l -p` lines: `0xID  DESKTOP  PID  HOST  TITLE...`
fn parse_wmctrl_lp(text: &str, windows: &mut Vec<WindowRecord>, pid_to_desktop: &mut HashMap<u32, u32>) {
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let _win_id = parts.next();
        let desktop = match parts.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(d) if d >= 0 => d as u32,
            _ => continue,
        };
        let pid = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        let _host = parts.next();
        let title = parts.collect::<Vec<_>>().join(" ");
        windows.push(WindowRecord {
            pid,
            desktop,
            title,
        });
        if pid > 10 {
            pid_to_desktop.entry(pid).or_insert(desktop);
        }
    }
}

#[cfg(target_os = "linux")]
fn read_ppid(pid: u32) -> Option<u32> {
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("PPid:\t") {
            return rest.trim().parse().ok();
        }
    }
    None
}

#[cfg(not(target_os = "linux"))]
fn read_ppid(_pid: u32) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_wmctrl_sample_lines() {
        let sample = "\
0x02800004  0 41573  pop-os  Obsidian
0x03a00007  1 34702  pop-os  LibreOffice Calc
0x03c00004  0 9086   pop-os TickTick
0x01200003  -1 0  pop-os  Sticky
0x05c00004  5 2      pop-os Project structure - Obsidian 1.12.7
";
        let mut windows = Vec::new();
        let mut map = HashMap::new();
        parse_wmctrl_lp(sample, &mut windows, &mut map);
        assert_eq!(map.get(&41573), Some(&0));
        assert_eq!(map.get(&34702), Some(&1));
        assert_eq!(map.get(&9086), Some(&0));
        assert!(map.get(&0).is_none());
        assert_eq!(windows.len(), 4);
    }

    #[test]
    fn workspace_for_title_hint_obsidian() {
        let mut index = WorkspaceIndex::default();
        index.windows.push(WindowRecord {
            pid: 2,
            desktop: 5,
            title: "Project structure - noteTaking - Obsidian 1.12.7".into(),
        });
        assert_eq!(
            index.workspace_for_title_hint("Obsidian"),
            Some(5)
        );
    }

    #[test]
    fn workspace_for_pid_miss() {
        let index = WorkspaceIndex::default();
        assert!(index.workspace_for_pid(999_999).is_none());
    }
}

//! Best-effort desktop workspace read (X11 via wmctrl, Wayland adapters). Does not move windows.

use std::collections::HashMap;
use std::process::Command;

/// One mapped top-level window from a window source (`wmctrl`, Wayland foreign-toplevel, …).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowRecord {
    pub pid: u32,
    /// 0-based workspace index when known (`wmctrl` always sets this; Wayland/Cosmic
    /// may leave it `None` when workspace protocols soft-fail).
    pub desktop: Option<u32>,
    pub title: String,
    /// Wayland `app_id` when known (foreign-toplevel); unused for `wmctrl` rows.
    pub app_id: Option<String>,
}

/// Origin of a batch of window records merged into [`WorkspaceIndex`].
#[derive(Debug, Clone)]
pub enum WindowSource {
    /// EWMH listing via `wmctrl -l -p` (X11 or XWayland clients).
    Wmctrl(Vec<WindowRecord>),
    /// Wayland foreign-toplevel (or Cosmic extension) snapshot.
    WaylandForeignToplevel(Vec<WindowRecord>),
}

impl WindowSource {
    fn into_records(self) -> Vec<WindowRecord> {
        match self {
            WindowSource::Wmctrl(v) | WindowSource::WaylandForeignToplevel(v) => v,
        }
    }
}

/// Merge window records from one or more sources into a single index.
/// Later sources append; first PID→desktop mapping wins.
/// Duplicate titles across sources are collapsed (prefer record with `app_id`, else richer PID).
pub fn merge_window_sources(sources: impl IntoIterator<Item = WindowSource>) -> WorkspaceIndex {
    let mut index = WorkspaceIndex::default();
    for source in sources {
        index.ingest_records(source.into_records());
    }
    index.dedupe_windows_by_title();
    index
}

/// Prefer window-first discovery when the merged index has mapped surfaces.
pub fn uses_window_first_discovery(index: &WorkspaceIndex) -> bool {
    !index.windows().is_empty()
}

/// PID → workspace index (0-based) when resolvable, plus raw window list for title matching.
#[derive(Debug, Default)]
pub struct WorkspaceIndex {
    pid_to_desktop: HashMap<u32, u32>,
    windows: Vec<WindowRecord>,
}

impl WorkspaceIndex {
    pub fn load() -> Self {
        let mut sources = Vec::new();
        // X11 primary and Wayland supplemental (XWayland clients only — not complete coverage).
        if let Some(records) = load_wmctrl_records() {
            sources.push(WindowSource::Wmctrl(records));
        }
        // Native Wayland top-levels (Cosmic and others advertising foreign-toplevel).
        // Cosmic may enrich `desktop`; GNOME/KWin/Hyprland use the same path without
        // fabricating workspace indices (see `adapters` module).
        let wayland = super::wayland_windows::load_wayland_foreign_toplevel();
        if !wayland.is_empty() {
            let family = super::adapters::detect_compositor_family();
            if !super::adapters::workspace_enrichment_expected(family) {
                eprintln!(
                    "[maestro] compositor={family:?}: window list ok; workspace enrichment not claimed"
                );
            }
            sources.push(WindowSource::WaylandForeignToplevel(wayland));
        }
        merge_window_sources(sources)
    }

    fn ingest_records(&mut self, records: Vec<WindowRecord>) {
        for record in records {
            if record.pid > 10 {
                if let Some(desktop) = record.desktop {
                    self.pid_to_desktop.entry(record.pid).or_insert(desktop);
                }
            }
            self.windows.push(record);
        }
    }

    /// Collapse the same top-level appearing from wmctrl + Wayland (identical title).
    fn dedupe_windows_by_title(&mut self) {
        let mut by_title: HashMap<String, WindowRecord> = HashMap::new();
        for record in std::mem::take(&mut self.windows) {
            let key = record.title.trim().to_lowercase();
            if key.is_empty() {
                self.windows.push(record);
                continue;
            }
            by_title
                .entry(key)
                .and_modify(|best| {
                    *best = prefer_window_record(best, &record);
                })
                .or_insert(record);
        }
        self.windows = by_title.into_values().collect();
        // Rebuild pid map from survivors.
        self.pid_to_desktop.clear();
        for w in &self.windows {
            if w.pid > 10 {
                if let Some(desktop) = w.desktop {
                    self.pid_to_desktop.entry(w.pid).or_insert(desktop);
                }
            }
        }
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
    ///
    /// Returns a workspace only when every title match has a known desktop and
    /// they all agree. Taking `.min()` across siblings (e.g. two Vivaldi windows
    /// on different workspaces) previously mis-grouped them; also refuse to guess
    /// when some matches still have `desktop=None`.
    pub fn workspace_for_title_hint(&self, hint: &str) -> Option<u32> {
        let hint = hint.trim();
        if hint.is_empty() {
            return None;
        }
        let lower = hint.to_lowercase();
        let matches: Vec<&WindowRecord> = self
            .windows
            .iter()
            .filter(|w| w.title.to_lowercase().contains(&lower))
            .collect();
        if matches.is_empty() {
            return None;
        }
        let mut desktops: Vec<u32> = Vec::with_capacity(matches.len());
        for w in &matches {
            let Some(d) = w.desktop else {
                // Partial mapping among siblings → ambiguous.
                return None;
            };
            desktops.push(d);
        }
        desktops.sort_unstable();
        desktops.dedup();
        if desktops.len() == 1 {
            Some(desktops[0])
        } else {
            None
        }
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

/// Prefer Wayland `app_id` rows over bare wmctrl duplicates; keep a usable PID when present.
fn prefer_window_record(a: &WindowRecord, b: &WindowRecord) -> WindowRecord {
    let a_score = window_record_quality(a);
    let b_score = window_record_quality(b);
    if b_score > a_score {
        merge_window_record_fields(b, a)
    } else {
        merge_window_record_fields(a, b)
    }
}

fn window_record_quality(w: &WindowRecord) -> i32 {
    let mut score = 0;
    if w.app_id.as_ref().is_some_and(|id| !id.is_empty()) {
        score += 10;
    }
    if w.pid > 10 {
        score += 3;
    }
    score
}

fn merge_window_record_fields(primary: &WindowRecord, other: &WindowRecord) -> WindowRecord {
    WindowRecord {
        pid: if primary.pid > 10 {
            primary.pid
        } else if other.pid > 10 {
            other.pid
        } else {
            primary.pid
        },
        // Prefer a known workspace; do not treat missing as index 0.
        desktop: primary.desktop.or(other.desktop),
        title: primary.title.clone(),
        app_id: primary.app_id.clone().or_else(|| other.app_id.clone()),
    }
}

/// Uses `wmctrl -l -p` subprocess (avoids pulling in x11rb). Soft-fails when absent.
fn load_wmctrl_records() -> Option<Vec<WindowRecord>> {
    let output = match Command::new("wmctrl").args(["-l", "-p"]).output() {
        Ok(o) if o.status.success() => o,
        _ => return None,
    };
    let text = String::from_utf8_lossy(&output.stdout);
    let mut windows = Vec::new();
    let mut pid_to_desktop = HashMap::new();
    parse_wmctrl_lp(&text, &mut windows, &mut pid_to_desktop);
    if windows.is_empty() {
        None
    } else {
        Some(windows)
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
            desktop: Some(desktop),
            title,
            app_id: None,
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
            desktop: Some(5),
            title: "Project structure - noteTaking - Obsidian 1.12.7".into(),
            app_id: None,
        });
        assert_eq!(
            index.workspace_for_title_hint("Obsidian"),
            Some(5)
        );
    }

    #[test]
    fn workspace_for_title_hint_ambiguous_multi_window_returns_none() {
        let mut index = WorkspaceIndex::default();
        index.windows.push(WindowRecord {
            pid: 0,
            desktop: Some(1),
            title: "(3) WhatsApp - Vivaldi".into(),
            app_id: Some("vivaldi-stable".into()),
        });
        index.windows.push(WindowRecord {
            pid: 0,
            desktop: Some(3),
            title: "Cursor Grok 4.5: Uso Pro - Google Gemini - Vivaldi".into(),
            app_id: Some("vivaldi-stable".into()),
        });
        // Must not pick .min() (workspace 1) when siblings disagree.
        assert_eq!(index.workspace_for_title_hint("Vivaldi"), None);
        assert_eq!(index.workspace_for_executable("/opt/vivaldi/vivaldi", "Vivaldi"), None);
    }

    #[test]
    fn title_hint_refuses_when_sibling_lacks_desktop() {
        // Regression: Gemini Vivaldi with desktop=None was assigned WhatsApp's WS.
        let mut index = WorkspaceIndex::default();
        index.windows.push(WindowRecord {
            pid: 0,
            desktop: Some(1),
            title: "(3) WhatsApp - Vivaldi".into(),
            app_id: Some("vivaldi-stable".into()),
        });
        index.windows.push(WindowRecord {
            pid: 0,
            desktop: None,
            title: "Cursor Grok 4.5: Uso Pro - Google Gemini - Vivaldi".into(),
            app_id: Some("vivaldi-stable".into()),
        });
        assert_eq!(index.workspace_for_title_hint("Vivaldi"), None);
        assert_eq!(index.workspace_for_executable("/opt/vivaldi/vivaldi", "Vivaldi"), None);
        let gemini = index
            .windows
            .iter()
            .find(|w| w.title.contains("Gemini"))
            .expect("gemini");
        assert_eq!(gemini.desktop, None);
    }

    #[test]
    fn workspace_for_title_hint_agrees_when_same_desktop() {
        let mut index = WorkspaceIndex::default();
        index.windows.push(WindowRecord {
            pid: 0,
            desktop: Some(2),
            title: "Tab A - Vivaldi".into(),
            app_id: None,
        });
        index.windows.push(WindowRecord {
            pid: 0,
            desktop: Some(2),
            title: "Tab B - Vivaldi".into(),
            app_id: None,
        });
        assert_eq!(index.workspace_for_title_hint("Vivaldi"), Some(2));
    }

    #[test]
    fn workspace_for_pid_miss() {
        let index = WorkspaceIndex::default();
        assert!(index.workspace_for_pid(999_999).is_none());
    }

    #[test]
    fn merge_window_sources_combines_records_and_pid_map() {
        let wmctrl = vec![WindowRecord {
            pid: 100,
            desktop: Some(0),
            title: "Slack".into(),
            app_id: None,
        }];
        let wayland = vec![WindowRecord {
            pid: 200,
            desktop: Some(2),
            title: "Firefox".into(),
            app_id: Some("firefox".into()),
        }];
        let merged = merge_window_sources([
            WindowSource::Wmctrl(wmctrl),
            WindowSource::WaylandForeignToplevel(wayland),
        ]);
        assert_eq!(merged.windows().len(), 2);
        assert_eq!(merged.workspace_for_pid(100), Some(0));
        assert_eq!(merged.workspace_for_pid(200), Some(2));
        assert!(merged.windows().iter().any(|w| w.title == "Slack"));
        assert!(merged.windows().iter().any(|w| w.title == "Firefox"));
    }

    #[test]
    fn merge_empty_sources_yields_empty_index() {
        let merged = merge_window_sources([
            WindowSource::Wmctrl(vec![]),
            WindowSource::WaylandForeignToplevel(vec![]),
        ]);
        assert!(merged.windows().is_empty());
        assert!(!uses_window_first_discovery(&merged));
    }

    #[test]
    fn non_empty_merged_index_selects_window_first() {
        let merged = merge_window_sources([WindowSource::Wmctrl(vec![WindowRecord {
            pid: 42,
            desktop: Some(1),
            title: "App".into(),
            app_id: None,
        }])]);
        assert!(uses_window_first_discovery(&merged));
    }

    #[test]
    fn merge_dedupes_same_title_preferring_wayland_app_id() {
        let merged = merge_window_sources([
            WindowSource::Wmctrl(vec![WindowRecord {
                pid: 4,
                desktop: Some(0),
                title: "novo-website - Slack".into(),
                app_id: None,
            }]),
            WindowSource::WaylandForeignToplevel(vec![WindowRecord {
                pid: 0,
                desktop: None,
                title: "novo-website - Slack".into(),
                app_id: Some("Slack".into()),
            }]),
        ]);
        assert_eq!(merged.windows().len(), 1);
        assert_eq!(merged.windows()[0].app_id.as_deref(), Some("Slack"));
        // Prefer known wmctrl desktop when Wayland row lacks workspace.
        assert_eq!(merged.windows()[0].desktop, Some(0));
    }

    #[test]
    fn merge_prefers_known_wayland_desktop_over_none() {
        let merged = merge_window_sources([
            WindowSource::Wmctrl(vec![WindowRecord {
                pid: 4,
                desktop: Some(1),
                title: "Firefox".into(),
                app_id: None,
            }]),
            WindowSource::WaylandForeignToplevel(vec![WindowRecord {
                pid: 0,
                desktop: Some(2),
                title: "Firefox".into(),
                app_id: Some("firefox".into()),
            }]),
        ]);
        assert_eq!(merged.windows().len(), 1);
        // Wayland wins on quality; its known desktop is kept.
        assert_eq!(merged.windows()[0].desktop, Some(2));
    }

    #[test]
    #[ignore = "requires live Wayland/Cosmic session with open windows"]
    fn live_load_merges_wayland_or_wmctrl_windows() {
        let index = WorkspaceIndex::load();
        let n = index.windows().len();
        println!("[live] WorkspaceIndex::load windows={n}");
        for w in index.windows().iter().take(12) {
            println!(
                "  pid={} desktop={:?} app_id={:?} title={}",
                w.pid, w.desktop, w.app_id, w.title
            );
        }
        assert!(n > 0, "expected merged windows on Cosmic with apps open");
        let slack: Vec<_> = index
            .windows()
            .iter()
            .filter(|w| w.title.contains("Slack"))
            .collect();
        assert_eq!(slack.len(), 1, "Slack must not be duplicated across sources");
    }
}

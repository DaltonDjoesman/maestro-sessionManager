//! Window-first running app discovery with scored classification.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;

use sysinfo::{
    Pid, Process, ProcessesToUpdate, ProcessRefreshKind, System, UpdateKind,
};

use crate::capture::assistant::{
    assistant_background_noise, assistant_basename_skip_for_capture,
    assistant_chromium_subprocess_cmd, cwd_hint_for_editor, process_cmdline_full,
    CandidateKind, ClassificationConfidence, RunningAppCandidate,
};
use crate::capture::classifier::{classify, flatpak_snap_path_hint, user_session_active, ScoreInput};
use crate::capture::desktop_index::{humanize_basename, DesktopIndex};
use crate::platform::{denylisted_basename, has_resolved_executable, session_type, WindowRecord, WorkspaceIndex};

const MAX_CANDIDATES: usize = 200;

pub fn discover_running_app_candidates() -> Vec<RunningAppCandidate> {
    let desktop_index = DesktopIndex::load();
    let workspace_index = WorkspaceIndex::load();
    let st = session_type();

    let mut out = if (st == "x11" || st == "tty") && !workspace_index.windows().is_empty() {
        discover_from_windows(&desktop_index, &workspace_index)
    } else {
        Vec::new()
    };

    if out.is_empty() {
        out = discover_from_processes(&desktop_index, &workspace_index);
    }

    out.sort_by(|a, b| display_name_of(a).cmp(&display_name_of(b)));
    if out.len() > MAX_CANDIDATES {
        out.truncate(MAX_CANDIDATES);
    }
    out
}

fn discover_from_windows(
    desktop: &DesktopIndex,
    workspace: &WorkspaceIndex,
) -> Vec<RunningAppCandidate> {
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
    for window in workspace.windows() {
        if is_wm_noise_title(&window.title) {
            continue;
        }
        if let Some(c) = candidate_from_window(window, desktop, workspace, &sys, my_pid) {
            out.push(c);
        }
    }

    dedupe_candidates(out)
}

fn candidate_from_window(
    window: &WindowRecord,
    desktop: &DesktopIndex,
    workspace: &WorkspaceIndex,
    sys: &System,
    my_pid: u32,
) -> Option<RunningAppCandidate> {
    if window.pid > 10 {
        if let Some(proc) = sys.process(Pid::from_u32(window.pid)) {
            return build_candidate(
                window.pid,
                proc,
                Some(window),
                desktop,
                workspace,
                my_pid,
                true,
            );
        }
    }

    build_candidate_from_title(window, desktop, workspace, sys, my_pid)
}

fn build_candidate_from_title(
    window: &WindowRecord,
    desktop: &DesktopIndex,
    workspace: &WorkspaceIndex,
    sys: &System,
    my_pid: u32,
) -> Option<RunningAppCandidate> {
    let desktop_entry = desktop.match_window_title(&window.title);

    for (pid, proc) in sys.processes() {
        let pid_u32 = pid.as_u32();
        if pid_u32 <= 1 || pid_u32 == my_pid {
            continue;
        }
        if !has_resolved_executable(proc) {
            continue;
        }
        let executable = executable_of(proc);
        let basename = basename_of(proc, &executable);
        if denylisted_basename(&basename) {
            continue;
        }
        let cmd_full = process_cmdline_full(proc.cmd());
        if assistant_chromium_subprocess_cmd(&cmd_full)
            || assistant_background_noise(&executable, &cmd_full)
        {
            continue;
        }
        let matches = if let Some(entry) = desktop_entry {
            desktop.match_process(&executable, &basename, Some(&cmd_full)) == Some(entry)
                || window
                    .title
                    .to_lowercase()
                    .contains(&entry.name.to_lowercase())
        } else {
            window.title.to_lowercase().contains(&basename)
                || desktop
                    .match_process(&executable, &basename, Some(&cmd_full))
                    .is_some()
        };
        if matches {
            return build_candidate(
                pid_u32,
                proc,
                Some(window),
                desktop,
                workspace,
                my_pid,
                true,
            );
        }
    }

    let entry = desktop_entry?;
    let score_input = ScoreInput {
        has_window: true,
        desktop_match: true,
        startup_wm_class_match: entry.startup_wm_class.is_some(),
        user_session: user_session_active(),
        ..Default::default()
    };
    let (kind, confidence, _) = classify(&score_input);
    if kind != CandidateKind::App {
        return None;
    }
    let exe = entry.exec_keys.first()?.clone();
    Some(RunningAppCandidate {
        pid: window.pid.max(11),
        executable: exe.clone(),
        label: basename_from_path(&exe),
        cmd_preview: window.title.clone(),
        cwd_hint: None,
        kind,
        display_name: entry.name.clone(),
        icon_name: entry.icon.clone(),
        desktop_workspace: Some(window.desktop),
        window_title: Some(window.title.clone()),
        classification_confidence: Some(confidence),
    })
}

fn discover_from_processes(
    desktop: &DesktopIndex,
    workspace: &WorkspaceIndex,
) -> Vec<RunningAppCandidate> {
    let my_pid = std::process::id();
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing()
            .with_exe(UpdateKind::Always)
            .with_cmd(UpdateKind::Always)
            .with_memory(),
    );

    let mut out = Vec::new();
    for (pid, proc) in sys.processes() {
        let pid_u32 = pid.as_u32();
        if pid_u32 <= 1 || pid_u32 == my_pid {
            continue;
        }
        if let Some(c) = build_candidate(
            pid_u32,
            proc,
            None,
            desktop,
            workspace,
            my_pid,
            workspace.has_window_for_pid(pid_u32)
                || workspace.workspace_for_pid(pid_u32).is_some(),
        ) {
            out.push(c);
        }
    }

    let mut out = dedupe_candidates(out);
    for c in &mut out {
        if c.desktop_workspace.is_none() {
            if let Some(d) = workspace.workspace_for_pid(c.pid) {
                c.desktop_workspace = Some(d);
            } else if let Some(d) =
                workspace.workspace_for_executable(&c.executable, &c.display_name)
            {
                c.desktop_workspace = Some(d);
            }
        }
    }
    out
}

fn build_candidate(
    pid_u32: u32,
    proc: &Process,
    window: Option<&WindowRecord>,
    desktop: &DesktopIndex,
    workspace: &WorkspaceIndex,
    my_pid: u32,
    has_window: bool,
) -> Option<RunningAppCandidate> {
    let _ = my_pid;
    let raw_name = proc.name().to_string_lossy();
    if raw_name.starts_with('[') || !has_resolved_executable(proc) {
        return None;
    }

    let executable = executable_of(proc);
    let basename = basename_of(proc, &executable);
    if denylisted_basename(&basename) {
        return None;
    }

    let cmd_full = process_cmdline_full(proc.cmd());
    if assistant_chromium_subprocess_cmd(&cmd_full)
        || assistant_background_noise(&executable, &cmd_full)
        || assistant_basename_skip_for_capture(&executable)
    {
        return None;
    }

    let label = proc
        .exe()
        .and_then(|p| p.file_name().and_then(OsStr::to_str))
        .map(String::from)
        .unwrap_or_else(|| raw_name.to_string());

    let cmd_preview = if !proc.cmd().is_empty() {
        proc.cmd()
            .iter()
            .map(|a| a.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .take(200)
            .collect()
    } else {
        raw_name.to_string()
    };

    let desktop_entry = desktop.match_process(&executable, &basename, Some(&cmd_full));
    let wm_match = window
        .and_then(|w| desktop.match_window_title(&w.title))
        .is_some();
    let window_mapped = has_window
        || window.is_some()
        || workspace.has_window_for_pid(pid_u32)
        || workspace.workspace_for_pid(pid_u32).is_some();

    let score_input = ScoreInput {
        excluded: false,
        has_window: window_mapped,
        desktop_match: desktop_entry.is_some(),
        startup_wm_class_match: wm_match,
        flatpak_snap_hint: flatpak_snap_path_hint(&executable),
        user_session: user_session_active(),
    };
    let (kind, confidence, _) = classify(&score_input);

    let (display_name, icon_name) = if let Some(entry) = desktop_entry {
        (entry.name.clone(), entry.icon.clone())
    } else if let Some(w) = window {
        if let Some(entry) = desktop.match_window_title(&w.title) {
            (entry.name.clone(), entry.icon.clone())
        } else {
            (humanize_basename(&label), None)
        }
    } else {
        (humanize_basename(&label), None)
    };

    let desktop_workspace = window
        .map(|w| w.desktop)
        .or_else(|| workspace.workspace_for_pid(pid_u32))
        .or_else(|| workspace.workspace_for_executable(&executable, &display_name));

    let window_title = window.map(|w| w.title.clone());

    Some(RunningAppCandidate {
        pid: pid_u32,
        executable,
        label,
        cmd_preview,
        cwd_hint: cwd_hint_for_editor(&basename, pid_u32),
        kind,
        display_name,
        icon_name,
        desktop_workspace,
        window_title,
        classification_confidence: Some(confidence),
    })
}

fn dedupe_candidates(candidates: Vec<RunningAppCandidate>) -> Vec<RunningAppCandidate> {
    let mut by_key: HashMap<String, RunningAppCandidate> = HashMap::new();
    for c in candidates {
        let key = candidate_dedupe_key(&c);
        by_key
            .entry(key)
            .and_modify(|best| merge_candidate(best, &c))
            .or_insert(c);
    }
    by_key.into_values().collect()
}

fn merge_candidate(best: &mut RunningAppCandidate, incoming: &RunningAppCandidate) {
    if incoming.desktop_workspace.is_some() && best.desktop_workspace.is_none() {
        best.desktop_workspace = incoming.desktop_workspace;
    }
    if best.window_title.is_none() {
        best.window_title = incoming.window_title.clone();
    }
    if best.classification_confidence == Some(ClassificationConfidence::Low)
        && incoming.classification_confidence != Some(ClassificationConfidence::Low)
    {
        best.classification_confidence = incoming.classification_confidence.clone();
    }
    if best.kind == CandidateKind::Process && incoming.kind == CandidateKind::App {
        best.kind = CandidateKind::App;
        best.display_name = incoming.display_name.clone();
        best.icon_name = incoming.icon_name.clone();
    }
}

fn candidate_dedupe_key(c: &RunningAppCandidate) -> String {
    const EDITORS: &[&str] = &["cursor", "code", "code-oss", "codium", "obsidian"];
    let base = Path::new(&c.executable)
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    let ws = c
        .desktop_workspace
        .map(|w| w.to_string())
        .unwrap_or_else(|| "none".into());
    let title = c.window_title.as_deref().unwrap_or("");
    if EDITORS.contains(&base.as_str()) {
        let folder = c.cwd_hint.as_deref().unwrap_or("");
        format!("{}\x1f{}\x1f{}\x1f{}", c.executable, folder, ws, title)
    } else {
        format!("{}\x1f{}\x1f{}", c.executable, ws, title)
    }
}

fn is_wm_noise_title(title: &str) -> bool {
    let t = title.trim();
    t.is_empty() || t.starts_with("@!")
}

fn executable_of(proc: &Process) -> String {
    proc.exe()
        .map(|p| p.to_string_lossy().into_owned())
        .filter(|s| !s.is_empty())
        .or_else(|| {
            proc.cmd()
                .first()
                .map(|a| a.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| proc.name().to_string_lossy().into_owned())
}

fn basename_of(proc: &Process, executable: &str) -> String {
    proc.exe()
        .and_then(|p| p.file_name().and_then(OsStr::to_str))
        .map(|s| s.to_lowercase())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            Path::new(executable)
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_else(|| proc.name().to_string_lossy().to_lowercase())
        })
}

fn basename_from_path(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
        .to_string()
}

fn display_name_of(c: &RunningAppCandidate) -> String {
    c.display_name.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedupe_key_splits_same_exe_by_workspace_and_title() {
        let a = RunningAppCandidate {
            pid: 1,
            executable: "/usr/bin/vivaldi".into(),
            label: "vivaldi".into(),
            cmd_preview: String::new(),
            cwd_hint: None,
            kind: CandidateKind::App,
            display_name: "Vivaldi".into(),
            icon_name: None,
            desktop_workspace: Some(1),
            window_title: Some("Tab A".into()),
            classification_confidence: Some(ClassificationConfidence::High),
        };
        let b = RunningAppCandidate {
            pid: 1,
            executable: "/usr/bin/vivaldi".into(),
            label: "vivaldi".into(),
            cmd_preview: String::new(),
            cwd_hint: None,
            kind: CandidateKind::App,
            display_name: "Vivaldi".into(),
            icon_name: None,
            desktop_workspace: Some(3),
            window_title: Some("Tab B".into()),
            classification_confidence: Some(ClassificationConfidence::High),
        };
        assert_ne!(candidate_dedupe_key(&a), candidate_dedupe_key(&b));
    }

    #[test]
    fn wm_noise_title_skipped() {
        assert!(is_wm_noise_title("@!1920,0;BDHF"));
        assert!(!is_wm_noise_title("TickTick"));
    }
}

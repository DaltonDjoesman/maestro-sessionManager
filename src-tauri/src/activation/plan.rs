//! Single-sourced activation plan: warnings, browser argv, application argv/skip — used by preview and execute.

use serde::{Deserialize, Serialize};

use crate::browser::build_browser_launch;
use crate::process_launcher::{resolved_spawn_argv, LaunchSpec};
use crate::profiles::SessionProfile;

use super::skip::{collect_running_executable_basenames, entry_skip_detail};
use super::validate::validate_before_activation;
use super::ActivationError;

#[derive(Debug, Clone)]
pub(crate) enum PlannedActivationItem {
    BrowserWarning(String),
    BrowserLaunch { program: String, args: Vec<String> },
    Application {
        spec: LaunchSpec,
        skip_detail: Option<String>,
    },
}

pub(crate) fn build_planned_activation(
    profile: &SessionProfile,
    path_label: &str,
) -> Result<Vec<PlannedActivationItem>, ActivationError> {
    validate_before_activation(profile, path_label)?;
    let profile = profile.clone().normalize();
    let running = collect_running_executable_basenames();
    let mut items = Vec::new();

    for entry in &profile.applications {
        let skip_detail = entry_skip_detail(entry, &running);
        if let Some(block) = entry.to_profile_browser_block() {
            if skip_detail.is_some() {
                items.push(PlannedActivationItem::Application {
                    spec: LaunchSpec::from(entry),
                    skip_detail,
                });
                continue;
            }
            let built = build_browser_launch(&block)?;
            for w in &built.warnings {
                items.push(PlannedActivationItem::BrowserWarning(w.clone()));
            }
            let (program, args) = built.argv_for_spawn();
            items.push(PlannedActivationItem::BrowserLaunch {
                program: program.to_string(),
                args: args.to_vec(),
            });
        } else {
            let spec = LaunchSpec::from(entry);
            items.push(PlannedActivationItem::Application {
                spec,
                skip_detail,
            });
        }
    }

    Ok(items)
}

/// Serializable preview row for the UI / `preview_session_activation` command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivationPreviewStep {
    pub step_type: String,
    pub label: String,
    pub argv: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub would_skip: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_detail: Option<String>,
}

pub(crate) fn preview_steps_from_plan(items: &[PlannedActivationItem]) -> Vec<ActivationPreviewStep> {
    let mut out = Vec::new();
    for item in items {
        match item {
            PlannedActivationItem::BrowserWarning(message) => {
                out.push(ActivationPreviewStep {
                    step_type: "browser_warning".into(),
                    label: "Warning".into(),
                    argv: vec![],
                    cwd: None,
                    would_skip: None,
                    skip_detail: Some(message.clone()),
                });
            }
            PlannedActivationItem::BrowserLaunch { program, args } => {
                let mut argv = vec![program.clone()];
                argv.extend(args.iter().cloned());
                out.push(ActivationPreviewStep {
                    step_type: "browser".into(),
                    label: "Browser".into(),
                    argv,
                    cwd: None,
                    would_skip: None,
                    skip_detail: None,
                });
            }
            PlannedActivationItem::Application { spec, skip_detail } => {
                let (prog, args) = resolved_spawn_argv(spec);
                let mut argv = vec![prog];
                argv.extend(args);
                let cwd = spec
                    .cwd
                    .as_ref()
                    .map(|p| p.to_string_lossy().into_owned());
                let would_skip = skip_detail.is_some();
                out.push(ActivationPreviewStep {
                    step_type: "application".into(),
                    label: spec.executable.clone(),
                    argv,
                    cwd,
                    would_skip: Some(would_skip),
                    skip_detail: skip_detail.clone(),
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{ApplicationBrowserSettings, ApplicationLaunchEntry, SessionProfile};
    use crate::settings::BrowserFamily;

    #[test]
    fn preview_argv_matches_resolved_spawn_for_single_app() {
        let p = SessionProfile {
            schema_version: 1,
            session_id: "sid".into(),
            name: "N".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: "/bin/true".into(),
                args: vec!["--foo".into()],
                cwd: Some("/tmp".into()),
                skip_if_running: None,
                browser: None,
            }],
            browser: None,
            cleanup: None,
        };
        let plan = build_planned_activation(&p, "/x.json").expect("plan");
        let preview = preview_steps_from_plan(&plan);
        assert_eq!(preview.len(), 1);
        let spec = LaunchSpec::from(&p.applications[0]);
        let (prog, args) = resolved_spawn_argv(&spec);
        let mut expected = vec![prog];
        expected.extend(args);
        assert_eq!(preview[0].argv, expected);
    }

    #[test]
    fn flatpak_rewrite_preview_matches_resolved_spawn() {
        let p = SessionProfile {
            schema_version: 1,
            session_id: "sid".into(),
            name: "N".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: "/app/obsidian".into(),
                args: vec!["--verbose".into()],
                cwd: None,
                skip_if_running: None,
                browser: None,
            }],
            browser: None,
            cleanup: None,
        };
        let plan = build_planned_activation(&p, "/x.json").expect("plan");
        let preview = preview_steps_from_plan(&plan);
        let spec = LaunchSpec::from(&p.applications[0]);
        let (prog, args) = resolved_spawn_argv(&spec);
        let mut expected = vec![prog];
        expected.extend(args);
        assert_eq!(preview[0].argv, expected);
        assert_eq!(preview[0].argv[0], "flatpak");
    }

    #[test]
    fn browser_preview_includes_program_in_argv() {
        let p = SessionProfile {
            schema_version: 1,
            session_id: "sid".into(),
            name: "N".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: "/bin/true".into(),
                args: vec![],
                cwd: None,
                skip_if_running: None,
                browser: Some(ApplicationBrowserSettings {
                    family: BrowserFamily::ChromiumLike,
                    user_data_dir: Some("/tmp/ud".into()),
                    firefox_profile: None,
                    firefox_no_remote: None,
                    urls: vec!["https://a.example".into()],
                }),
            }],
            browser: None,
            cleanup: None,
        };
        let plan = build_planned_activation(&p, "/x.json").expect("plan");
        let preview = preview_steps_from_plan(&plan);
        let browser_steps: Vec<_> = preview
            .iter()
            .filter(|s| s.step_type == "browser")
            .collect();
        assert_eq!(browser_steps.len(), 1);
        assert_eq!(browser_steps[0].argv[0], "/bin/true");
        assert!(browser_steps[0].argv.contains(&"--new-window".to_string()));
    }
}

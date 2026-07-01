//! Orchestrate browser then applications; emit structured step summaries (`session-activation` spec).

use std::path::Path;
use std::time::Duration;

use crate::process_launcher::{spawn_command, spawn_launch_spec, LaunchSpec, SpawnOutcome};
use crate::profiles::SessionProfile;

use super::log::{log_line, try_open_activation_log};
use super::plan::{build_planned_activation, preview_steps_from_plan, PlannedActivationItem};
use super::{
    ActivateSessionResult, ActivationError, ActivationStepKind, ActivationStepStatus,
    ActivationStepSummary,
};

use super::plan::ActivationPreviewStep;

/// Build the activation plan and map it to preview rows (no I/O, no spawns).
pub fn preview_session_activation(
    profile: &SessionProfile,
    path_label: &str,
) -> Result<Vec<ActivationPreviewStep>, ActivationError> {
    let plan = build_planned_activation(profile, path_label)?;
    Ok(preview_steps_from_plan(&plan))
}

/// Validate, then launch browser (if any), then applications in profile order.
pub async fn activate_session_profile(
    profile: &SessionProfile,
    path_label: &str,
) -> Result<ActivateSessionResult, ActivationError> {
    let plan = build_planned_activation(profile, path_label)?;
    let mut log_path_str: Option<String> = None;
    let mut log_file = match try_open_activation_log() {
        Some((p, mut f)) => {
            log_path_str = Some(p.to_string_lossy().into_owned());
            log_line(&mut f, &format!("profile={path_label}"));
            Some(f)
        }
        None => None,
    };

    let mut steps = Vec::new();
    let delay_after_success = Duration::ZERO;

    let mut app_index: usize = 0;
    let total_apps = profile.applications.len();

    for item in plan {
        match item {
            PlannedActivationItem::BrowserWarning(message) => {
                if let Some(ref mut f) = log_file {
                    log_line(f, &format!("warning: {message}"));
                }
                steps.push(ActivationStepSummary {
                    kind: ActivationStepKind::Browser,
                    label: "Browser".into(),
                    status: ActivationStepStatus::Warning,
                    detail: Some(message),
                    pid: None,
                });
            }
            PlannedActivationItem::BrowserLaunch { program, args } => {
                if let Some(ref mut f) = log_file {
                    log_line(f, &format!("spawn browser: {} {:?}", program, args));
                }
                let cwd_path = None::<&Path>;
                let outcome = spawn_command(&program, &args, cwd_path).await;
                if let Some(ref mut f) = log_file {
                    log_line(f, &format!("browser outcome: {outcome:?}"));
                }
                steps.push(browser_row(&outcome));
            }
            PlannedActivationItem::Application {
                spec,
                skip_detail,
            } => {
                if let Some(detail) = skip_detail {
                    if let Some(ref mut f) = log_file {
                        log_line(f, &format!("skip app {}: {detail}", spec.executable));
                    }
                    steps.push(ActivationStepSummary {
                        kind: ActivationStepKind::Application,
                        label: spec.executable.clone(),
                        status: ActivationStepStatus::Skipped,
                        detail: Some(detail),
                        pid: None,
                    });
                    app_index += 1;
                    continue;
                }

                if let Some(ref mut f) = log_file {
                    let (p, a) = crate::process_launcher::resolved_spawn_argv(&spec);
                    log_line(
                        f,
                        &format!("spawn app {} -> {} {:?}", spec.executable, p, a),
                    );
                }
                let outcome = spawn_launch_spec(&spec).await;
                if let Some(ref mut f) = log_file {
                    log_line(f, &format!("app {} outcome: {outcome:?}", spec.executable));
                }
                steps.push(application_row(&spec, &outcome));

                let success = matches!(outcome, SpawnOutcome::Started { .. });
                let more = app_index + 1 < total_apps;
                if more && success && delay_after_success > Duration::ZERO {
                    let running = crate::activation::skip::collect_running_executable_basenames();
                    let any_following_spawn = profile.applications[app_index + 1..]
                        .iter()
                        .any(|e| {
                            crate::activation::skip::entry_skip_detail(e, &running).is_none()
                        });
                    if any_following_spawn {
                        tokio::time::sleep(delay_after_success).await;
                    }
                }
                app_index += 1;
            }
        }
    }

    Ok(ActivateSessionResult {
        steps,
        activation_log_path: log_path_str,
    })
}

fn browser_row(outcome: &SpawnOutcome) -> ActivationStepSummary {
    match outcome {
        SpawnOutcome::Started { pid } => ActivationStepSummary {
            kind: ActivationStepKind::Browser,
            label: "Browser".into(),
            status: ActivationStepStatus::Success,
            detail: None,
            pid: Some(*pid),
        },
        SpawnOutcome::Failed { message } => ActivationStepSummary {
            kind: ActivationStepKind::Browser,
            label: "Browser".into(),
            status: ActivationStepStatus::Failure,
            detail: Some(message.clone()),
            pid: None,
        },
    }
}

fn application_row(spec: &LaunchSpec, outcome: &SpawnOutcome) -> ActivationStepSummary {
    match outcome {
        SpawnOutcome::Started { pid } => ActivationStepSummary {
            kind: ActivationStepKind::Application,
            label: spec.executable.clone(),
            status: ActivationStepStatus::Success,
            detail: None,
            pid: Some(*pid),
        },
        SpawnOutcome::Failed { message } => ActivationStepSummary {
            kind: ActivationStepKind::Application,
            label: spec.executable.clone(),
            status: ActivationStepStatus::Failure,
            detail: Some(message.clone()),
            pid: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profiles::{ApplicationLaunchEntry, ProfileBrowserBlock, SessionProfile};
    use crate::settings::BrowserFamily;

    #[tokio::test]
    async fn invalid_profile_errors_before_spawn() {
        let p = SessionProfile {
            schema_version: 1,
            session_id: "id".into(),
            name: "n".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: "".into(),
                args: vec![],
                cwd: None,
                skip_if_running: None,
            }],
            browser: None,
            cleanup: None,
        };
        let err = activate_session_profile(&p, "/x.json").await.unwrap_err();
        assert!(matches!(err, ActivationError::Profile(_)));
    }

    #[tokio::test]
    async fn launches_one_application_without_browser() {
        let p = SessionProfile {
            schema_version: 1,
            session_id: "sid".into(),
            name: "N".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: "/bin/true".into(),
                args: vec![],
                cwd: None,
                skip_if_running: None,
            }],
            browser: None,
            cleanup: None,
        };
        let out = activate_session_profile(&p, "/ok.json").await.expect("ok");
        assert_eq!(out.steps.len(), 1);
        assert_eq!(out.steps[0].kind, ActivationStepKind::Application);
        assert_eq!(out.steps[0].status, ActivationStepStatus::Success);
        assert!(out.steps[0].pid.is_some());
    }

    #[tokio::test]
    async fn browser_then_app_two_steps() {
        let p = SessionProfile {
            schema_version: 1,
            session_id: "sid".into(),
            name: "N".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: "/bin/true".into(),
                args: vec![],
                cwd: None,
                skip_if_running: None,
            }],
            browser: Some(ProfileBrowserBlock {
                family: BrowserFamily::ChromiumLike,
                executable: "/bin/true".into(),
                user_data_dir: Some("/tmp/maestro-test-ud".into()),
                firefox_profile: None,
                firefox_no_remote: None,
                urls: vec![],
            }),
            cleanup: None,
        };
        let out = activate_session_profile(&p, "/ok.json").await.expect("ok");
        let steps = &out.steps;
        assert!(
            steps.len() >= 2,
            "expected browser row + app row, got {steps:?}"
        );
        assert_eq!(steps[0].kind, ActivationStepKind::Browser);
        assert_eq!(steps[steps.len() - 1].kind, ActivationStepKind::Application);
    }

    #[tokio::test]
    async fn preview_argv_matches_post_validation_plan() {
        let p = SessionProfile {
            schema_version: 1,
            session_id: "sid".into(),
            name: "N".into(),
            applications: vec![ApplicationLaunchEntry {
                executable: "/bin/sh".into(),
                args: vec!["-c".into(), "exit 0".into()],
                cwd: Some("/".into()),
                skip_if_running: None,
            }],
            browser: None,
            cleanup: None,
        };
        let prev = preview_session_activation(&p, "/ok.json").expect("preview");
        let out = activate_session_profile(&p, "/ok.json").await.expect("ok");
        assert_eq!(prev.len(), out.steps.len());
        let spec = LaunchSpec::from(&p.applications[0]);
        let (prog, args) = crate::process_launcher::resolved_spawn_argv(&spec);
        let mut expected_argv = vec![prog];
        expected_argv.extend(args);
        assert_eq!(prev[0].argv, expected_argv);
    }
}

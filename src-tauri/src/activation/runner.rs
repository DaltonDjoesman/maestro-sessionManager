//! Orchestrate browser then applications; emit structured step summaries (`session-activation` spec).

use std::path::Path;
use std::time::Duration;

use crate::browser::build_browser_launch;
use crate::process_launcher::{spawn_command, spawn_launch_spec, LaunchSpec, SpawnOutcome};
use crate::profiles::SessionProfile;

use super::skip::{collect_running_executable_basenames, entry_skip_detail};
use super::validate::validate_before_activation;
use super::{
    ActivationError, ActivationStepKind, ActivationStepStatus, ActivationStepSummary,
};

/// Validate, then launch browser (if any), then applications in profile order.
pub async fn activate_session_profile(
    profile: &SessionProfile,
    path_label: &str,
) -> Result<Vec<ActivationStepSummary>, ActivationError> {
    validate_before_activation(profile, path_label)?;

    let running = collect_running_executable_basenames();
    let mut steps = Vec::new();

    if let Some(browser) = profile.browser.as_ref() {
        let built = build_browser_launch(browser)?;
        for w in &built.warnings {
            steps.push(ActivationStepSummary {
                kind: ActivationStepKind::Browser,
                label: "Browser".into(),
                status: ActivationStepStatus::Warning,
                detail: Some(w.clone()),
                pid: None,
            });
        }
        let (program, args) = built.argv_for_spawn();
        let cwd_path = None::<&Path>;
        let outcome = spawn_command(program, args, cwd_path).await;
        steps.push(browser_row(&outcome));
    }

    let delay_after_success = Duration::ZERO;
    for (i, entry) in profile.applications.iter().enumerate() {
        if let Some(detail) = entry_skip_detail(entry, &running) {
            steps.push(ActivationStepSummary {
                kind: ActivationStepKind::Application,
                label: entry.executable.clone(),
                status: ActivationStepStatus::Skipped,
                detail: Some(detail),
                pid: None,
            });
            continue;
        }

        let spec = LaunchSpec::from(entry);
        let outcome = spawn_launch_spec(&spec).await;
        steps.push(application_row(&spec, &outcome));

        let success = matches!(outcome, SpawnOutcome::Started { .. });
        let more = i + 1 < profile.applications.len();
        if more && success && delay_after_success > Duration::ZERO {
            let any_following_spawn = profile.applications[i + 1..]
                .iter()
                .any(|e| entry_skip_detail(e, &running).is_none());
            if any_following_spawn {
                tokio::time::sleep(delay_after_success).await;
            }
        }
    }

    Ok(steps)
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
        let steps = activate_session_profile(&p, "/ok.json").await.expect("ok");
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].kind, ActivationStepKind::Application);
        assert_eq!(steps[0].status, ActivationStepStatus::Success);
        assert!(steps[0].pid.is_some());
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
        let steps = activate_session_profile(&p, "/ok.json").await.expect("ok");
        assert!(
            steps.len() >= 2,
            "expected browser row + app row, got {steps:?}"
        );
        assert_eq!(steps[0].kind, ActivationStepKind::Browser);
        assert_eq!(
            steps[steps.len() - 1].kind,
            ActivationStepKind::Application
        );
    }
}

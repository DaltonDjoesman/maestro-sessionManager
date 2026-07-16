//! Map raw spawn outcomes into activation summary rows for the UI.

use std::time::Duration;

use crate::activation::{
    ActivationStepKind, ActivationStepStatus, ActivationStepSummary,
};

use super::sequence::spawn_ordered_with_delay_after_success;
use super::{LaunchSpec, SpawnOutcome};

/// Zip parallel slices from [`spawn_ordered_with_delay_after_success`] into UI-facing rows.
#[allow(dead_code)] // Covered by unit tests; reserved for delayed multi-app launch wiring.
pub fn map_application_spawn_results(
    specs: &[LaunchSpec],
    outcomes: &[SpawnOutcome],
) -> Vec<ActivationStepSummary> {
    assert_eq!(
        specs.len(),
        outcomes.len(),
        "specs and outcomes must have the same length"
    );
    specs
        .iter()
        .zip(outcomes.iter())
        .map(|(spec, outcome)| match outcome {
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
        })
        .collect()
}

/// Run ordered application spawns and build activation summaries in one call (for task 6 wiring).
#[allow(dead_code)] // Covered by unit tests; reserved for delayed multi-app launch wiring.
pub async fn spawn_application_sequence_summaries(
    specs: &[LaunchSpec],
    delay_after_success: Duration,
) -> Vec<ActivationStepSummary> {
    let outcomes =
        spawn_ordered_with_delay_after_success(specs, delay_after_success).await;
    map_application_spawn_results(specs, &outcomes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_started_and_failed() {
        let specs = vec![
            LaunchSpec {
                executable: "/bin/true".into(),
                args: vec![],
                cwd: None,
            },
            LaunchSpec {
                executable: "/bad/exec".into(),
                args: vec![],
                cwd: None,
            },
        ];
        let outcomes = vec![
            SpawnOutcome::Started { pid: 42 },
            SpawnOutcome::Failed {
                message: "nope".into(),
            },
        ];
        let rows = map_application_spawn_results(&specs, &outcomes);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].status, ActivationStepStatus::Success);
        assert_eq!(rows[0].pid, Some(42));
        assert_eq!(rows[1].status, ActivationStepStatus::Failure);
        assert_eq!(rows[1].detail.as_deref(), Some("nope"));
    }

    #[tokio::test]
    async fn end_to_end_summaries_match_spawn_order() {
        let specs = vec![
            LaunchSpec {
                executable: "/bin/true".into(),
                args: vec![],
                cwd: None,
            },
            LaunchSpec {
                executable: "/no/such/bin-maestro-zzz".into(),
                args: vec![],
                cwd: None,
            },
        ];
        let rows = spawn_application_sequence_summaries(&specs, Duration::ZERO).await;
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].status, ActivationStepStatus::Success);
        assert_eq!(rows[1].status, ActivationStepStatus::Failure);
        assert!(rows[0].pid.is_some());
    }
}

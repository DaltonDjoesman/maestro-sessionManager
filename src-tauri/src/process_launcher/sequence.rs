//! Ordered launches with optional delay after each successful start.

use std::time::Duration;

use super::spawn::{spawn_launch_spec, LaunchSpec, SpawnOutcome};

/// Launch each spec in order. After each **successful** start, waits `delay_after_success`
/// before attempting the next entry (skipped when delay is zero or this was the last entry).
pub async fn spawn_ordered_with_delay_after_success(
    specs: &[LaunchSpec],
    delay_after_success: Duration,
) -> Vec<SpawnOutcome> {
    let mut results = Vec::with_capacity(specs.len());
    for (i, spec) in specs.iter().enumerate() {
        let outcome = spawn_launch_spec(spec).await;
        let success = matches!(outcome, SpawnOutcome::Started { .. });
        results.push(outcome);
        let more = i + 1 < specs.len();
        if more && success && delay_after_success > Duration::ZERO {
            tokio::time::sleep(delay_after_success).await;
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn true_specs(n: usize) -> Vec<LaunchSpec> {
        (0..n)
            .map(|_| LaunchSpec {
                executable: "/bin/true".into(),
                args: vec![],
                cwd: None,
            })
            .collect()
    }

    #[tokio::test]
    async fn three_successes_sleep_between_yields_at_least_two_delays() {
        let specs = true_specs(3);
        let delay = Duration::from_millis(80);
        let start = Instant::now();
        let out = spawn_ordered_with_delay_after_success(&specs, delay).await;
        assert_eq!(out.len(), 3);
        assert!(out.iter().all(|o| matches!(o, SpawnOutcome::Started { .. })));
        assert!(
            start.elapsed() >= delay * 2,
            "expected ~2 gaps between 3 successes, got {:?}",
            start.elapsed()
        );
    }

    #[tokio::test]
    async fn zero_delay_runs_quickly() {
        let specs = true_specs(5);
        let start = Instant::now();
        let _ = spawn_ordered_with_delay_after_success(&specs, Duration::ZERO).await;
        assert!(
            start.elapsed() < Duration::from_millis(500),
            "zero delay should not add noticeable sleep"
        );
    }

    #[tokio::test]
    async fn failure_then_continue_without_blocking_activation_plan() {
        let specs = vec![
            LaunchSpec {
                executable: "/bin/true".into(),
                args: vec![],
                cwd: None,
            },
            LaunchSpec {
                executable: "/no/such/maestro-missing-bin".into(),
                args: vec![],
                cwd: None,
            },
            LaunchSpec {
                executable: "/bin/true".into(),
                args: vec![],
                cwd: None,
            },
        ];
        let out = spawn_ordered_with_delay_after_success(&specs, Duration::from_millis(10)).await;
        assert_eq!(out.len(), 3);
        assert!(matches!(out[0], SpawnOutcome::Started { .. }));
        assert!(matches!(out[1], SpawnOutcome::Failed { .. }));
        assert!(matches!(out[2], SpawnOutcome::Started { .. }));
    }
}

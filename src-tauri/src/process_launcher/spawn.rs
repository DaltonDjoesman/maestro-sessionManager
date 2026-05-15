//! Spawn child processes from launch specs (executable, args, optional cwd).

use std::path::PathBuf;
use std::process::Stdio;

use tokio::process::Command;

/// One application launch line item (mirrors profile JSON fields).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchSpec {
    pub executable: String,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
}

/// Result of attempting to start a detached child process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpawnOutcome {
    Started { pid: u32 },
    Failed { message: String },
}

/// Spawn without blocking the async executor; I/O is discarded; spawn errors are captured.
pub async fn spawn_launch_spec(spec: &LaunchSpec) -> SpawnOutcome {
    let mut cmd = Command::new(&spec.executable);
    cmd.args(&spec.args);
    if let Some(cwd) = &spec.cwd {
        cmd.current_dir(cwd);
    }
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());

    match cmd.spawn() {
        Ok(child) => {
            let pid = child.id().unwrap_or(0);
            drop(child);
            SpawnOutcome::Started { pid }
        }
        Err(e) => SpawnOutcome::Failed {
            message: format!("failed to spawn `{}`: {e}", spec.executable),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn spawn_bin_true_succeeds() {
        let spec = LaunchSpec {
            executable: "/bin/true".into(),
            args: vec![],
            cwd: None,
        };
        let out = spawn_launch_spec(&spec).await;
        assert!(
            matches!(out, SpawnOutcome::Started { pid } if pid > 0),
            "unexpected outcome: {out:?}"
        );
    }

    #[tokio::test]
    async fn spawn_missing_executable_fails_with_message() {
        let spec = LaunchSpec {
            executable: "/no/such/maestro-test-binary-xyz".into(),
            args: vec![],
            cwd: None,
        };
        let out = spawn_launch_spec(&spec).await;
        assert!(
            matches!(out, SpawnOutcome::Failed { .. }),
            "expected failure: {out:?}"
        );
    }

    #[tokio::test]
    async fn spawn_with_args() {
        let spec = LaunchSpec {
            executable: "/bin/sh".into(),
            args: vec!["-c".into(), "exit 0".into()],
            cwd: None,
        };
        let out = spawn_launch_spec(&spec).await;
        assert!(matches!(out, SpawnOutcome::Started { .. }), "{out:?}");
    }
}

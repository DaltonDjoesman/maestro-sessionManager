//! Spawn child processes from launch specs (executable, args, optional cwd).

use std::path::{Path, PathBuf};
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

/// Program + arguments exactly as [`spawn_launch_spec`] would use (including Flatpak host rewrite).
pub fn resolved_spawn_argv(spec: &LaunchSpec) -> (String, Vec<String>) {
    if let Some((program, argv)) = try_flatpak_host_launch(spec) {
        return (program, argv);
    }
    // Replaced installs leave `/proc/.../exe` as `path (deleted)`; recover via PATH basename.
    let program = crate::platform::resolve_launch_executable(&spec.executable);
    (program, spec.args.clone())
}

/// Flatpak apps expose `/app/<name>` inside the sandbox; that path does not exist on the host.
/// Map known basenames to `flatpak run <app-id>` (profile `args` are appended after the app id).
fn try_flatpak_host_launch(spec: &LaunchSpec) -> Option<(String, Vec<String>)> {
    let exe = spec.executable.trim();
    if !exe.starts_with("/app/") {
        return None;
    }
    if Path::new(exe).is_file() {
        return None;
    }
    let base = Path::new(exe)
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())?;
    let app_id = match base.as_str() {
        "obsidian" => "md.obsidian.Obsidian",
        _ => return None,
    };
    let mut argv = vec!["run".into(), app_id.into()];
    argv.extend(spec.args.iter().cloned());
    Some(("flatpak".into(), argv))
}

/// Spawn without blocking the async executor; I/O is discarded; spawn errors are captured.
pub async fn spawn_launch_spec(spec: &LaunchSpec) -> SpawnOutcome {
    let (program, argv) = resolved_spawn_argv(spec);
    if program != spec.executable || argv != spec.args {
        return spawn_command(&program, &argv, spec.cwd.as_deref()).await;
    }

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

/// Spawn a program with explicit argv slice (used by session activation for browser argv).
pub async fn spawn_command(program: &str, args: &[String], cwd: Option<&Path>) -> SpawnOutcome {
    let mut cmd = Command::new(program);
    cmd.args(args);
    if let Some(cwd) = cwd {
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
            message: format!("failed to spawn `{program}`: {e}"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatpak_host_maps_obsidian_sandbox_path() {
        let spec = LaunchSpec {
            executable: "/app/obsidian".into(),
            args: vec!["--foo".into()],
            cwd: None,
        };
        let (prog, argv) = resolved_spawn_argv(&spec);
        assert_eq!(prog, "flatpak");
        assert_eq!(argv, vec!["run", "md.obsidian.Obsidian", "--foo"]);
    }

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

    #[tokio::test]
    async fn spawn_command_matches_launch_spec() {
        let out = spawn_command("/bin/true", &[], None).await;
        assert!(matches!(out, SpawnOutcome::Started { .. }), "{out:?}");
    }
}

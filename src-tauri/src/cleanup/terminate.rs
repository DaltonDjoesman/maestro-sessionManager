//! SIGTERM / optional timed SIGKILL for explicit cleanup confirmations (`context-cleanup` spec).

use std::time::Duration;

use sysinfo::{Pid, ProcessesToUpdate, ProcessRefreshKind, Signal, System};

/// Refuse obviously unsafe targets.
pub fn validate_user_cleanup_pid(pid: u32) -> Result<(), String> {
    if pid <= 1 {
        return Err("refusing signals for pid 0 or 1".into());
    }
    if pid == std::process::id() {
        return Err("refusing to signal the Maestro process itself".into());
    }
    Ok(())
}

/// Send SIGTERM, then optionally wait and SIGKILL processes that remain.
pub async fn terminate_process(
    pid: u32,
    force_kill_after_ms: Option<u64>,
) -> Result<String, String> {
    validate_user_cleanup_pid(pid)?;

    let mut sys = System::new();
    let pid_sys = Pid::from_u32(pid);
    sys.refresh_processes_specifics(
        ProcessesToUpdate::Some(std::slice::from_ref(&pid_sys)),
        true,
        ProcessRefreshKind::nothing(),
    );

    let proc = sys
        .process(pid_sys)
        .ok_or_else(|| format!("process {pid} not found"))?;

    match proc.kill_with(Signal::Term) {
        None => return Err("SIGTERM is not supported on this platform".into()),
        Some(false) => return Err(format!("failed to send SIGTERM to pid {pid}")),
        Some(true) => {}
    }

    let mut detail = format!("SIGTERM sent to pid {pid}");

    if let Some(ms) = force_kill_after_ms {
        tokio::time::sleep(Duration::from_millis(ms)).await;
        sys.refresh_processes_specifics(
            ProcessesToUpdate::Some(std::slice::from_ref(&pid_sys)),
            true,
            ProcessRefreshKind::nothing(),
        );
        if let Some(p) = sys.process(pid_sys) {
            match p.kill_with(Signal::Kill) {
                None => detail.push_str("; SIGKILL unsupported on this platform"),
                Some(false) => detail.push_str("; SIGKILL failed"),
                Some(true) => {
                    detail.push_str(&format!("; SIGKILL sent after {ms} ms (still running)"));
                }
            }
        } else {
            detail.push_str(&format!("; exited before SIGKILL after {ms} ms"));
        }
    }

    Ok(detail)
}

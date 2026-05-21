//! Serializable rows for cleanup diff and termination results (Tauri / UI).

use serde::Serialize;

use crate::platform::ProcessCandidate;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CleanupDivergenceRow {
    pub pid: u32,
    pub executable_basename: String,
    pub cmd_preview: String,
}

impl From<&ProcessCandidate> for CleanupDivergenceRow {
    fn from(c: &ProcessCandidate) -> Self {
        Self {
            pid: c.pid,
            executable_basename: c.executable_basename.clone(),
            cmd_preview: c.cmd_preview.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CleanupTerminateResult {
    pub pid: u32,
    pub ok: bool,
    pub message: String,
}

//! Structured activation step results for the UI (`session-activation` spec).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActivationStepKind {
    Application,
    Browser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActivationStepStatus {
    Success,
    Failure,
    Warning,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivationStepSummary {
    pub kind: ActivationStepKind,
    pub label: String,
    pub status: ActivationStepStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,
}

/// Result of executing an activation (steps plus optional log file on disk).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateSessionResult {
    pub steps: Vec<ActivationStepSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_log_path: Option<String>,
}

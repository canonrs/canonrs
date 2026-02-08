// @canon-level: strict
// @canon-rules: [#21, #24, #36, #37]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Active,
    Completed,
    Blocked,
    Skipped,
    Failed,
    NotApplicable,
}

impl fmt::Display for StepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Active => write!(f, "active"),
            Self::Completed => write!(f, "completed"),
            Self::Blocked => write!(f, "blocked"),
            Self::Skipped => write!(f, "skipped"),
            Self::Failed => write!(f, "failed"),
            Self::NotApplicable => write!(f, "not_applicable"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GateType {
    Evidence,
    Permission,
    Rule,
    Temporal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StepId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub step_id: StepId,
    pub from_status: StepStatus,
    pub to_status: StepStatus,
    pub timestamp: String,
    pub actor: Option<String>,
    pub reason: Option<String>,
}

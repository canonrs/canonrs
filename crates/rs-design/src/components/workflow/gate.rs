// @canon-level: strict
// @canon-rules: [#36, #37]
use crate::components::workflow::types::{GateType, StepId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gate {
    pub gate_type: GateType,
    pub required: Vec<String>,
    pub blocking: bool,
    pub reason: Option<String>,
}

impl Gate {
    pub fn evidence(required: Vec<String>) -> Self {
        Self {
            gate_type: GateType::Evidence,
            required,
            blocking: true,
            reason: None,
        }
    }

    pub fn permission(required: Vec<String>) -> Self {
        Self {
            gate_type: GateType::Permission,
            required,
            blocking: true,
            reason: None,
        }
    }

    pub fn rule(required: Vec<String>, reason: Option<String>) -> Self {
        Self {
            gate_type: GateType::Rule,
            required,
            blocking: true,
            reason,
        }
    }

    pub fn temporal(required: Vec<String>, reason: Option<String>) -> Self {
        Self {
            gate_type: GateType::Temporal,
            required,
            blocking: true,
            reason,
        }
    }

    pub fn is_satisfied(&self, evidence: &[String]) -> bool {
        self.required.iter().all(|r| evidence.contains(r))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_gate_satisfied() {
        let gate = Gate::evidence(vec!["doc1".to_string()]);
        assert!(gate.is_satisfied(&["doc1".to_string()]));
        assert!(!gate.is_satisfied(&[]));
    }
}

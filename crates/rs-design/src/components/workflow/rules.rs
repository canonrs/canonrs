// @canon-level: strict
// @canon-rules: [#36, #37]
use crate::components::workflow::types::{StepId, StepStatus};
use std::collections::HashMap;

pub struct DependencyRule {
    pub step_id: StepId,
    pub depends_on: Vec<(StepId, StepStatus)>,
}

impl DependencyRule {
    pub fn is_satisfied(&self, states: &HashMap<StepId, StepStatus>) -> bool {
        self.depends_on.iter().all(|(dep_id, required_status)| {
            states.get(dep_id).map_or(false, |status| status == required_status)
        })
    }
}

pub fn can_transition(from: StepStatus, to: StepStatus) -> bool {
    match (from, to) {
        // Pending pode ir para qualquer estado
        (StepStatus::Pending, StepStatus::Active) => true,
        (StepStatus::Pending, StepStatus::Skipped) => true,
        (StepStatus::Pending, StepStatus::NotApplicable) => true,
        
        // Active pode completar, falhar ou ser bloqueado
        (StepStatus::Active, StepStatus::Completed) => true,
        (StepStatus::Active, StepStatus::Failed) => true,
        (StepStatus::Active, StepStatus::Blocked) => true,
        
        // Blocked pode voltar a Active
        (StepStatus::Blocked, StepStatus::Active) => true,
        
        // Completed pode ser resetado (volta para Pending)
        (StepStatus::Completed, StepStatus::Pending) => true,
        
        // Failed pode ser resetado
        (StepStatus::Failed, StepStatus::Pending) => true,
        (StepStatus::Failed, StepStatus::Active) => true,
        
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(can_transition(StepStatus::Pending, StepStatus::Active));
        assert!(can_transition(StepStatus::Active, StepStatus::Completed));
        assert!(can_transition(StepStatus::Completed, StepStatus::Pending)); // Reset
        assert!(can_transition(StepStatus::Active, StepStatus::Failed));
        assert!(!can_transition(StepStatus::Completed, StepStatus::Active));
    }
}

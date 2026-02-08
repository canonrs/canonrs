use crate::components::workflow::{can_transition, StepStatus};

#[test]
fn test_pending_to_active() {
    assert!(can_transition(StepStatus::Pending, StepStatus::Active));
}

#[test]
fn test_active_to_completed() {
    assert!(can_transition(StepStatus::Active, StepStatus::Completed));
}

#[test]
fn test_invalid_completed_to_pending() {
    assert!(!can_transition(StepStatus::Completed, StepStatus::Pending));
}

#[test]
fn test_blocked_to_active() {
    assert!(can_transition(StepStatus::Blocked, StepStatus::Active));
}

#[test]
fn test_pending_to_skipped() {
    assert!(can_transition(StepStatus::Pending, StepStatus::Skipped));
}

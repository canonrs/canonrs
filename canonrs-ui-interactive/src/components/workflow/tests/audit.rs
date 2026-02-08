use crate::components::workflow::{AuditTrail, StepId, StepStatus};

#[test]
fn test_audit_trail_records() {
    let trail = AuditTrail::new(None);
    trail.record_transition(
        StepId("step1".to_string()),
        StepStatus::Pending,
        StepStatus::Active,
        Some("user123".to_string()),
        None,
    );
    assert_eq!(trail.get_entries().len(), 1);
}

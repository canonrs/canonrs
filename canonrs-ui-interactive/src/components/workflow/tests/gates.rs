use crate::components::workflow::{Gate, GateType};

#[test]
fn test_evidence_gate_creation() {
    let gate = Gate::evidence(vec!["doc1".to_string()]);
    assert_eq!(gate.gate_type, GateType::Evidence);
    assert!(gate.blocking);
}

#[test]
fn test_gate_satisfaction() {
    let gate = Gate::evidence(vec!["doc1".to_string(), "doc2".to_string()]);
    assert!(!gate.is_satisfied(&["doc1".to_string()]));
    assert!(gate.is_satisfied(&["doc1".to_string(), "doc2".to_string()]));
}

#[test]
fn test_permission_gate() {
    let gate = Gate::permission(vec!["admin".to_string()]);
    assert_eq!(gate.gate_type, GateType::Permission);
}

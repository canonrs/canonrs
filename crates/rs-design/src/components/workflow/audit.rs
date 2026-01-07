// @canon-level: strict
// @canon-rules: [#36, #37]
use crate::components::workflow::types::{AuditEntry, StepId, StepStatus};
use leptos::prelude::*;

pub type AuditCallback = Callback<AuditEntry>;

#[derive(Clone)]
pub struct AuditTrail {
    entries: RwSignal<Vec<AuditEntry>>,
    on_audit: Option<AuditCallback>,
}

impl AuditTrail {
    pub fn new(on_audit: Option<AuditCallback>) -> Self {
        Self {
            entries: RwSignal::new(Vec::new()),
            on_audit,
        }
    }

    pub fn record_transition(
        &self,
        step_id: StepId,
        from_status: StepStatus,
        to_status: StepStatus,
        actor: Option<String>,
        reason: Option<String>,
    ) {
        let entry = AuditEntry {
            step_id,
            from_status,
            to_status,
            timestamp: chrono::Utc::now().to_rfc3339(),
            actor,
            reason,
        };

        self.entries.update(|entries: &mut Vec<AuditEntry>| entries.push(entry.clone()));

        if let Some(cb) = &self.on_audit {
            cb.run(entry);
        }
    }

    pub fn get_entries(&self) -> Vec<AuditEntry> {
        self.entries.get()
    }
}

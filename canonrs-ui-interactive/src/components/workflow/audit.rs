use leptos::callback::Callback;
use crate::components::workflow::types::{AuditEntry, StepId, StepStatus};
use leptos::prelude::*;
use std::sync::Arc;

pub type AuditCallback = Callback<AuditEntry>;

#[derive(Clone, Copy)]
pub struct AuditTrail {
    entries: RwSignal<Vec<AuditEntry>>,
    on_audit: RwSignal<Option<Arc<dyn Fn(AuditEntry) + Send + Sync>>>,
}

impl AuditTrail {
    pub fn new(on_audit: Option<AuditCallback>) -> Self {
        Self {
            entries: RwSignal::new(Vec::new()),
            on_audit: RwSignal::new(on_audit.map(|cb| {
                Arc::new(move |entry: AuditEntry| cb.run(entry)) as Arc<dyn Fn(AuditEntry) + Send + Sync>
            })),
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
        
        self.entries.update(|entries| {
            entries.push(entry.clone());
        });
        
        if let Some(cb) = self.on_audit.get() {
            cb(entry);
        }
    }
    
    pub fn get_entries(&self) -> Vec<AuditEntry> {
        self.entries.get()
    }
}

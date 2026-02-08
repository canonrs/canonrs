use leptos::callback::Callback;
use crate::components::workflow::{
    audit::AuditTrail,
    rules::{can_transition, DependencyRule},
    types::{StepId, StepStatus, AuditEntry},
};
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Workflow(
    steps: RwSignal<HashMap<StepId, StepStatus>>,
    #[prop(default = vec![])] _dependencies: Vec<DependencyRule>,
    #[prop(optional)] on_audit: Option<Callback<AuditEntry>>,
    children: Children,
) -> impl IntoView {
    let audit_trail = AuditTrail::new(on_audit);
    let states = steps;

    provide_context(audit_trail);
    provide_context(states);

    view! {
        <nav
            role="navigation"
            attr:aria-label="Workflow steps"
            attr:data-component="workflow"
            class="w-full"
        >
            <ol
                role="list"
                class="flex flex-col gap-4"
            >
                {children()}
            </ol>
        </nav>
    }
}

#[derive(Clone, Copy)]
pub struct WorkflowContext {
    pub states: RwSignal<HashMap<StepId, StepStatus>>,
}

pub fn use_workflow_context() -> WorkflowContext {
    let states = use_context::<RwSignal<HashMap<StepId, StepStatus>>>()
        .expect("WorkflowContext must be provided");
    WorkflowContext { states }
}

pub fn transition_step(
    step_id: StepId,
    to_status: StepStatus,
    actor: Option<String>,
    reason: Option<String>,
) {
    let ctx = use_workflow_context();
    let audit = use_context::<AuditTrail>()
        .expect("AuditTrail must be provided");

    ctx.states.update(|states| {
        if let Some(current) = states.get(&step_id) {
            if can_transition(*current, to_status) {
                audit.record_transition(
                    step_id.clone(),
                    *current,
                    to_status,
                    actor,
                    reason,
                );
                states.insert(step_id, to_status);
            }
        }
    });
}

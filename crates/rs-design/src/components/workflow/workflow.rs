// @canon-level: strict
// @canon-rules: [#21, #24, #36, #37]
use crate::components::workflow::{
    audit::{AuditCallback, AuditTrail},
    rules::{can_transition, DependencyRule},
    types::{StepId, StepStatus},
};
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Workflow(
    #[prop(into)] steps: Signal<HashMap<StepId, StepStatus>>,
    #[prop(optional)] dependencies: Vec<DependencyRule>,
    #[prop(optional)] on_audit: Option<AuditCallback>,
    children: Children,
) -> impl IntoView {
    let audit_trail = AuditTrail::new(on_audit);
    provide_context(audit_trail.clone());
    provide_context(RwSignal::new(steps.get_untracked()));

    view! {
        <nav
            role="navigation"
            aria-label="Workflow steps"
            data-component="workflow"
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
        .expect("WorkflowContext not found");
    WorkflowContext { states }
}

pub fn transition_step(
    step_id: StepId,
    to_status: StepStatus,
    actor: Option<String>,
    reason: Option<String>,
) {
    let ctx = use_workflow_context();
    let audit = use_context::<AuditTrail>().expect("AuditTrail not found");

    ctx.states.update(|states: &mut HashMap<StepId, StepStatus>| {
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

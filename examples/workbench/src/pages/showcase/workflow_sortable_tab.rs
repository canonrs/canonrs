use leptos::prelude::*;
use rs_design::ui::drag_drop::SortableList;
use rs_design::components::workflow::{StepStatus, WorkflowStepItem};

#[derive(Clone, Debug, PartialEq)]
struct WorkflowStep {
    id: String,
    label: String,
    status: StepStatus,
}

#[component]
pub fn WorkflowSortableTab() -> impl IntoView {
    let (steps, set_steps) = signal(vec![
        WorkflowStep {
            id: "step-1".to_string(),
            label: "Code Review".to_string(),
            status: StepStatus::Completed,
        },
        WorkflowStep {
            id: "step-2".to_string(),
            label: "Approval".to_string(),
            status: StepStatus::Active,
        },
        WorkflowStep {
            id: "step-3".to_string(),
            label: "Deploy".to_string(),
            status: StepStatus::Pending,
        },
        WorkflowStep {
            id: "step-4".to_string(),
            label: "Monitoring".to_string(),
            status: StepStatus::Pending,
        },
    ]);

    let (log, set_log) = signal(Vec::<String>::new());

    let on_reorder = Callback::new(move |new_steps: Vec<WorkflowStep>| {
        set_steps.set(new_steps);
        set_log.update(|l| {
            l.insert(0, "✨ Steps reordered".to_string());
            if l.len() > 10 { l.truncate(10); }
        });
    });

    eprintln!("🟢 Rendering WorkflowSortableTab - {} steps", steps.get().len());
    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Sortable Workflows"</h2>
                <p class="text-muted-foreground">"Drag to reorder workflow steps (Canon Rule #49)"</p>
            </div>

            <div class="grid grid-cols-3 gap-6">
                <div class="col-span-2 border rounded-lg p-6">
                    <h3 class="font-semibold mb-4">"🔄 Workflow Steps"</h3>
                    
                    <p class="text-red-500 font-bold">"DEBUG: SortableList deve aparecer abaixo"</p>
                    <SortableList
                        items=steps
                        on_reorder=on_reorder
                        item_id=|step: &WorkflowStep| step.id.clone()
                        render=move |step: WorkflowStep| {
                            let status = step.status;
    eprintln!("🟢 Rendering WorkflowSortableTab - {} steps", steps.get().len());
                            view! {
                                <div class="mb-2">
                                    <WorkflowStepItem
                                        label=step.label
                                        status=Signal::derive(move || status)
                                    />
                                </div>
                            }
                        }
                    />
                </div>

                <div class="space-y-4">
                    <div class="border rounded-lg p-4">
                        <h3 class="font-semibold mb-4">"Event Log"</h3>
                        <div class="space-y-1 text-xs font-mono max-h-[200px] overflow-y-auto">
                            {move || log.get().into_iter().map(|msg| {
    eprintln!("🟢 Rendering WorkflowSortableTab - {} steps", steps.get().len());
                                view! { <div>{msg}</div> }
                            }).collect_view()}
                        </div>
                    </div>

                    <div class="p-4 bg-purple-50 border border-purple-200 rounded">
                        <p class="text-sm font-semibold text-purple-900 mb-2">"🎯 Current Order"</p>
                        <ol class="text-xs text-purple-700 space-y-1">
                            {move || steps.get().into_iter().enumerate().map(|(i, step)| {
    eprintln!("🟢 Rendering WorkflowSortableTab - {} steps", steps.get().len());
                                view! {
                                    <li>{format!("{}. {}", i + 1, step.label)}</li>
                                }
                            }).collect_view()}
                        </ol>
                    </div>

                    <div class="p-4 bg-green-50 border border-green-200 rounded">
                        <p class="text-sm font-semibold text-green-900 mb-2">"✅ Features"</p>
                        <ul class="text-xs text-green-700 space-y-1">
                            <li>"• Drag to reorder"</li>
                            <li>"• Visual feedback"</li>
                            <li>"• Event logging"</li>
                            <li>"• Status preserved"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}

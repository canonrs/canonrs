use leptos::prelude::*;
use super::database_server::WorkflowStep;

#[component]
pub fn WorkflowStepView(
    step: WorkflowStep,
    #[prop(into)] on_click: Callback<(String, String)>
) -> impl IntoView {
    let sid = step.step_id.clone();
    let status = step.status.clone();
    let (icon, color, actions) = match status.as_str() {
        "Completed" => ("✅", "border-green-500 bg-green-50", vec![("Reset", "Pending")]),
        "Active" => ("⏳", "border-blue-500 bg-blue-50", vec![("Complete", "Completed"), ("Fail", "Failed")]),
        "Blocked" => ("🔒", "border-red-500 bg-red-50", vec![("Unblock", "Active")]),
        "Pending" => ("⏸️", "border-gray-300 bg-gray-50", vec![("Start", "Active")]),
        "Failed" => ("❌", "border-red-600 bg-red-100", vec![("Reset", "Pending")]),
        _ => ("➖", "border-gray-400 bg-gray-100", vec![]),
    };
    
    view! {
        <div class=format!("border-l-4 {} pl-4 py-3 rounded-r", color)>
            <div class="flex items-center justify-between">
                <div>
                    <p class="font-semibold flex items-center gap-2">
                        <span class="text-xl">{icon}</span>
                        <span>{step.label}</span>
                    </p>
                    <p class="text-sm text-muted-foreground">{status}</p>
                </div>
                <div class="flex gap-2">
                    {actions.into_iter().map(|(label, ns)| {
                        let s = sid.clone();
                        let n = ns.to_string();
                        view! {
                            <button
                                class="px-3 py-1 text-sm rounded bg-primary text-primary-foreground hover:bg-primary/90"
                                on:click=move |_| on_click.run((s.clone(), n.clone()))
                            >
                                {label}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

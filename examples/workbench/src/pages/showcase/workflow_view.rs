use leptos::prelude::*;
use super::database_server::{fetch_workflow_steps, fetch_workflow_audit};

/// Workflow View - READ ONLY
#[component]
pub fn WorkflowView(
    #[prop(optional)] refresh_trigger: Option<ReadSignal<u32>>
) -> impl IntoView {
    let steps_resource = Resource::new(
        move || refresh_trigger.map(|r| r.get()).unwrap_or(0),
        |_| async { fetch_workflow_steps(1).await }
    );
    
    let audit_resource = Resource::new(
        move || refresh_trigger.map(|r| r.get()).unwrap_or(0),
        |_| async { fetch_workflow_audit(1).await }
    );
    
    view! {
        <div class="space-y-6">
            <Transition fallback=|| view! { <div>"Loading workflow..."</div> }>
                {move || steps_resource.get().map(|result| match result {
                    Ok(steps) => view! {
                        <div class="space-y-4">
                            {steps.into_iter().map(|step| {
                                let status = step.status.clone();
                                let status_display = status.clone();
                                let step_id = step.step_id.clone();
                                let (icon, color) = match status.as_str() {
                                    "Completed" => ("✅", "border-green-500 bg-green-50"),
                                    "Active" => ("⏳", "border-blue-500 bg-blue-50"),
                                    "Blocked" => ("🔒", "border-red-500 bg-red-50"),
                                    "Pending" => ("⏸️", "border-gray-300 bg-gray-50"),
                                    "Failed" => ("❌", "border-red-600 bg-red-100"),
                                    _ => ("➖", "border-gray-400 bg-gray-100"),
                                };
                                
                                view! {
                                    <div class=format!("border-l-4 {} pl-4 py-3 rounded-r", color)>
                                        <div class="flex items-center gap-2">
                                            <span class="text-xl">{icon}</span>
                                            <div>
                                                <p class="font-semibold">{step.label}</p>
                                                <p class="text-sm text-muted-foreground">{status_display}</p>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any(),
                    Err(_) => view! { <div class="text-red-500">"Error"</div> }.into_any()
                })}
            </Transition>
            
            <div class="border rounded-lg p-6 bg-muted/50 mt-4">
                <h3 class="font-semibold mb-2">"Audit Trail"</h3>
                <Transition fallback=|| view! { <div>"Loading..."</div> }>
                    {move || audit_resource.get().map(|result| match result {
                        Ok(entries) => view! {
                            <div class="space-y-1 text-sm font-mono max-h-40 overflow-y-auto">
                                {entries.into_iter().map(|e| {
                                    view! {
                                        <div class="flex justify-between gap-4">
                                            <span>{format!("{}: {} → {}", e.step_id, e.from_status, e.to_status)}</span>
                                            <span class="text-muted-foreground text-xs">{e.timestamp}</span>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any(),
                        Err(_) => view! { <div class="text-muted-foreground">"No audit"</div> }.into_any()
                    })}
                </Transition>
            </div>
        </div>
    }
}

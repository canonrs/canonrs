use leptos::prelude::*;
use super::database_server::transition_workflow_step;

/// Workflow Actions - WRITE ONLY
/// Emits commands, triggers transitions
/// Client-only (uses spawn_local), never in SSR
#[component]
pub fn WorkflowActions(
    step_id: String,
    current_status: String,
    #[prop(into)] on_transition_complete: Callback<()>
) -> impl IntoView {
    let actions = match current_status.as_str() {
        "Completed" => vec![("Reset", "Pending")],
        "Active" => vec![("Complete", "Completed"), ("Fail", "Failed")],
        "Blocked" => vec![("Unblock", "Active")],
        "Pending" => vec![("Start", "Active")],
        "Failed" => vec![("Reset", "Pending")],
        _ => vec![],
    };
    
    view! {
        <div class="flex gap-2">
            {actions.into_iter().map(|(label, new_status)| {
                let sid = step_id.clone();
                let ns = new_status.to_string();
                let on_complete = on_transition_complete;
                
                view! {
                    <button
                        class="px-3 py-1 text-sm rounded bg-primary text-primary-foreground hover:bg-primary/90"
                        on:click=move |_| {
                            let sid = sid.clone();
                            let ns = ns.clone();
                            
                            #[cfg(target_arch = "wasm32")]
                            {
                                use leptos::task::spawn_local;
                                spawn_local(async move {
                                    if transition_workflow_step(1, sid, ns, Some("demo-user".into())).await.is_ok() {
                                        on_complete.run(());
                                    }
                                });
                            }
                        }
                    >
                        {label}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

use leptos::prelude::*;

/// AuditEntry - Single audit trail entry
#[derive(Clone, Debug, PartialEq)]
pub struct AuditEntry {
    pub id: usize,
    pub step_id: String,
    pub from_status: String,
    pub to_status: String,
    pub actor: Option<String>,
    pub timestamp: String,
}

/// WorkflowAuditLog - Displays audit trail for workflow
/// 
/// **Type:** Pure Component (Type 1)
/// **Canon Rules:** #1 (Types), #6 (Visual State)
/// **Tokens:** 100% Canonical
#[component]
pub fn WorkflowAuditLog(
    /// Audit entries to display
    entries: Vec<AuditEntry>,
    /// Optional CSS classes
    #[prop(optional, into)]
    class: String,
    /// Maximum height (scrollable)
    #[prop(optional)]
    max_height: Option<&'static str>,
) -> impl IntoView {
    let height_class = max_height.unwrap_or("max-h-40");
    
    view! {
        <div class=format!("border rounded-lg p-6 bg-muted/50 {}", class)>
            <h3 class="font-semibold mb-2">"Audit Trail"</h3>
            <div class=format!("space-y-1 text-sm font-mono overflow-y-auto {}", height_class)>
                {if entries.is_empty() {
                    view! {
                        <div class="text-muted-foreground italic">"No audit entries"</div>
                    }.into_any()
                } else {
                    entries.into_iter().map(|entry| {
                        view! {
                            <div class="flex justify-between gap-4 py-1 border-b border-muted last:border-0">
                                <span class="flex-1">
                                    <span class="font-semibold">{entry.step_id}</span>
                                    {": "}
                                    <span class="text-muted-foreground">{entry.from_status}</span>
                                    {" → "}
                                    <span>{entry.to_status}</span>
                                </span>
                                <span class="text-muted-foreground text-xs whitespace-nowrap">
                                    {entry.timestamp}
                                </span>
                            </div>
                        }
                    }).collect_view().into_any()
                }}
            </div>
        </div>
    }
}

use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AuditEntry {
    pub id: usize,
    pub step_id: String,
    pub from_status: String,
    pub to_status: String,
    pub actor: Option<String>,
    pub timestamp: String,
}

#[component]
pub fn WorkflowAuditLog(
    entries: Vec<AuditEntry>,
    #[prop(optional)] class: Option<String>,
    #[prop(default = "max-h-40")] max_height: &'static str,
) -> impl IntoView {
    view! {
        <div class={format!("border rounded-lg p-6 bg-muted/50 {}", class.unwrap_or_default())}>
            <h3 class="font-semibold mb-2">"Audit Trail"</h3>
            <div class={format!("space-y-1 text-sm font-mono overflow-y-auto {}", max_height)}>
                {move || {
                    if entries.is_empty() {
                        view! {
                            <div class="text-muted-foreground italic">"No audit entries"</div>
                        }.into_any()
                    } else {
                        entries.clone().into_iter().map(|entry| view! {
                            <div class="flex justify-between gap-4 py-1 border-b border-muted last:border-0">
                                <span class="flex-1">
                                    <span class="font-semibold">{entry.step_id}</span>
                                    ": "
                                    <span class="text-muted-foreground">{entry.from_status}</span>
                                    " → "
                                    <span>{entry.to_status}</span>
                                </span>
                                <span class="text-muted-foreground text-xs whitespace-nowrap">
                                    {entry.timestamp}
                                </span>
                            </div>
                        }).collect_view().into_any()
                    }
                }}
            </div>
        </div>
    }
}

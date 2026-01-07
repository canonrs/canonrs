use leptos::prelude::*;
use super::WorkflowClient;
use rs_design::components::workflow::WorkflowDemo;

#[component]
pub fn WorkflowTab() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Workflow Components"</h2>
                <p class="text-muted-foreground">"Persisted (left) vs Ephemeral Demo (right)"</p>
            </div>
            
            <div class="grid grid-cols-2 gap-6">
                <div class="border rounded-lg p-6">
                    <h3 class="text-lg font-semibold mb-4">"Persisted (SQLite + CQRS)"</h3>
                    <WorkflowClient />
                </div>
                
                <div class="border rounded-lg p-6">
                    <h3 class="text-lg font-semibold mb-4">"Demo (Ephemeral)"</h3>
                    <WorkflowDemo />
                </div>
            </div>
        </div>
    }
}

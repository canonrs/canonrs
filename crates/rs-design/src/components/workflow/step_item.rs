use leptos::prelude::*;
use super::step::{StepData, StepStatus};

/// WorkflowStepItem - Renders a single workflow step
/// 
/// **Type:** Pure Component (Type 1)
/// **Canon Rules:** #1 (Types), #6 (Visual State)
/// **Tokens:** 100% Canonical
#[component]
pub fn WorkflowStepItem(
    /// Step label
    label: String,
    /// Step status (reactive)
    #[prop(into)]
    status: Signal<StepStatus>,
    /// Optional CSS classes
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    view! {
        <div class=move || {
            let (icon, color) = match status.get() {
                StepStatus::Completed => ("✅", "border-green-500 bg-green-50"),
                StepStatus::Active => ("⏳", "border-blue-500 bg-blue-50"),
                StepStatus::Blocked => ("🔒", "border-red-500 bg-red-50"),
                StepStatus::Pending => ("⏸️", "border-gray-300 bg-gray-50"),
                StepStatus::Failed => ("❌", "border-red-600 bg-red-100"),
            };
            format!("border-l-4 {} pl-4 py-3 rounded-r {}", color, class)
        }>
            <div class="flex items-center gap-2">
                <span class="text-xl">{move || match status.get() {
                    StepStatus::Completed => "✅",
                    StepStatus::Active => "⏳",
                    StepStatus::Blocked => "🔒",
                    StepStatus::Pending => "⏸️",
                    StepStatus::Failed => "❌",
                }}</span>
                <div>
                    <p class="font-semibold">{label}</p>
                    <p class="text-sm text-muted-foreground">{move || match status.get() {
                        StepStatus::Completed => "Completed",
                        StepStatus::Active => "Active",
                        StepStatus::Blocked => "Blocked",
                        StepStatus::Pending => "Pending",
                        StepStatus::Failed => "Failed",
                    }}</p>
                </div>
            </div>
        </div>
    }
}

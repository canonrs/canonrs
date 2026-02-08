use leptos::prelude::*;
use super::types::StepStatus;

#[derive(Debug, Clone, PartialEq)]
pub struct StepData {
    pub id: usize,
    pub step_id: String,
    pub label: String,
    pub status: StepStatus,
}

#[component]
pub fn Step(
    _id: usize,
    _step_id: String,
    label: String,
    status: StepStatus,
) -> impl IntoView {
    let (icon, color) = match status {
        StepStatus::Completed => ("✅", "border-green-500 bg-green-50"),
        StepStatus::Active => ("⏳", "border-blue-500 bg-blue-50"),
        StepStatus::Blocked => ("🔒", "border-red-500 bg-red-50"),
        StepStatus::Pending => ("⏸️", "border-gray-300 bg-gray-50"),
        StepStatus::Failed => ("❌", "border-red-600 bg-red-100"),
        StepStatus::Skipped => ("⏭️", "border-yellow-500 bg-yellow-50"),
        StepStatus::NotApplicable => ("➖", "border-gray-400 bg-gray-100"),
    };

    let status_text = match status {
        StepStatus::Completed => "Completed",
        StepStatus::Active => "Active",
        StepStatus::Blocked => "Blocked",
        StepStatus::Pending => "Pending",
        StepStatus::Failed => "Failed",
        StepStatus::Skipped => "Skipped",
        StepStatus::NotApplicable => "Not Applicable",
    };

    view! {
        <div class={format!("border-l-4 {} pl-4 py-3 rounded-r", color)}>
            <div class="flex items-center gap-2">
                <span class="text-xl">{icon}</span>
                <div>
                    <p class="font-semibold">{label}</p>
                    <p class="text-sm text-muted-foreground">{status_text}</p>
                </div>
            </div>
        </div>
    }
}

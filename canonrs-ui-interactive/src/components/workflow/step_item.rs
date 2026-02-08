use leptos::prelude::*;
use super::types::StepStatus;

#[component]
pub fn WorkflowStepItem(
    label: String,
    status: Signal<StepStatus>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let icon_and_color = Memo::new(move |_| {
        match status.get() {
            StepStatus::Completed => ("✅", "border-green-500 bg-green-50"),
            StepStatus::Active => ("⏳", "border-blue-500 bg-blue-50"),
            StepStatus::Blocked => ("🔒", "border-red-500 bg-red-50"),
            StepStatus::Pending => ("⏸️", "border-gray-300 bg-gray-50"),
            StepStatus::Failed => ("❌", "border-red-600 bg-red-100"),
            StepStatus::Skipped => ("⏭️", "border-yellow-500 bg-yellow-50"),
            StepStatus::NotApplicable => ("➖", "border-gray-400 bg-gray-100"),
        }
    });
    
    let status_text = Memo::new(move |_| {
        match status.get() {
            StepStatus::Completed => "Completed",
            StepStatus::Active => "Active",
            StepStatus::Blocked => "Blocked",
            StepStatus::Pending => "Pending",
            StepStatus::Failed => "Failed",
            StepStatus::Skipped => "Skipped",
            StepStatus::NotApplicable => "Not Applicable",
        }
    });
    
    view! {
        <div class={move || format!("border-l-4 {} pl-4 py-3 rounded-r {}", icon_and_color.get().1, class.clone().unwrap_or_default())}>
            <div class="flex items-center gap-2">
                <span class="text-xl">{move || icon_and_color.get().0}</span>
                <div>
                    <p class="font-semibold">{label}</p>
                    <p class="text-sm text-muted-foreground">{move || status_text.get()}</p>
                </div>
            </div>
        </div>
    }
}

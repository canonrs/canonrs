use leptos::prelude::*;

/// Step status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    Active,
    Completed,
    Blocked,
    Failed,
}

/// Step data structure
#[derive(Debug, Clone, PartialEq)]
pub struct StepData {
    pub id: usize,
    pub step_id: String,
    pub label: String,
    pub status: StepStatus,
}

/// Step component - renders a workflow step
/// This is the COMPONENT, StepData is the DATA
#[component]
pub fn Step(
    #[prop(into)] id: usize,
    #[prop(into)] step_id: String,
    #[prop(into)] label: String,
    status: StepStatus,
) -> impl IntoView {
    let (icon, color) = match status {
        StepStatus::Completed => ("✅", "border-green-500 bg-green-50"),
        StepStatus::Active => ("⏳", "border-blue-500 bg-blue-50"),
        StepStatus::Blocked => ("🔒", "border-red-500 bg-red-50"),
        StepStatus::Pending => ("⏸️", "border-gray-300 bg-gray-50"),
        StepStatus::Failed => ("❌", "border-red-600 bg-red-100"),
    };
    
    let status_text = match status {
        StepStatus::Completed => "Completed",
        StepStatus::Active => "Active",
        StepStatus::Blocked => "Blocked",
        StepStatus::Pending => "Pending",
        StepStatus::Failed => "Failed",
    };
    
    view! {
        <div class=format!("border-l-4 {} pl-4 py-3 rounded-r", color)>
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

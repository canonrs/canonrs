use leptos::callback::Callback;
use leptos::prelude::*;
use super::{StepStatus, WorkflowStepItem};
use crate::ui::button::Button;

#[component]
pub fn WorkflowDemo() -> impl IntoView {
    let step1_status = RwSignal::new(StepStatus::Completed);
    let step2_status = RwSignal::new(StepStatus::Active);
    let step3_status = RwSignal::new(StepStatus::Blocked);
    let step4_status = RwSignal::new(StepStatus::Pending);

    view! {
        <div class="space-y-6">
            <div>
                <h2 class="text-2xl font-bold mb-2">"Workflow Demo (Ephemeral)"</h2>
                <p class="text-muted-foreground">"Interactive showcase - fully reactive"</p>
            </div>

            <div class="space-y-4">
                // Step 1
                <div class="flex items-center gap-4">
                    <div class="flex-1">
                        <WorkflowStepItem label="Code Review".to_string() status={step1_status.into()} />
                    </div>
                    <div class="flex gap-2">
                        {render_step_buttons(step1_status, Callback::new(move |s| step1_status.set(s)))}
                    </div>
                </div>

                // Step 2
                <div class="flex items-center gap-4">
                    <div class="flex-1">
                        <WorkflowStepItem label="Approval".to_string() status={step2_status.into()} />
                    </div>
                    <div class="flex gap-2">
                        {render_step_buttons(step2_status, Callback::new(move |s| step2_status.set(s)))}
                    </div>
                </div>

                // Step 3
                <div class="flex items-center gap-4">
                    <div class="flex-1">
                        <WorkflowStepItem label="Deploy".to_string() status={step3_status.into()} />
                    </div>
                    <div class="flex gap-2">
                        {render_step_buttons(step3_status, Callback::new(move |s| step3_status.set(s)))}
                    </div>
                </div>

                // Step 4
                <div class="flex items-center gap-4">
                    <div class="flex-1">
                        <WorkflowStepItem label="Monitoring".to_string() status={step4_status.into()} />
                    </div>
                    <div class="flex gap-2">
                        {render_step_buttons(step4_status, Callback::new(move |s| step4_status.set(s)))}
                    </div>
                </div>
            </div>

            <div class="p-4 bg-blue-50 border border-blue-200 rounded">
                <p class="text-sm font-semibold text-blue-900">"✅ Demo Mode - Fully Reactive"</p>
                <p class="text-xs text-blue-700 mt-1">"Click buttons to see real-time visual updates"</p>
            </div>
        </div>
    }
}

fn render_step_buttons(
    status: RwSignal<StepStatus>,
    on_change: Callback<StepStatus>,
) -> impl IntoView {
    move || match status.get() {
        StepStatus::Completed => view! {
            <Button
                class={"px-3 py-1 text-sm rounded bg-gray-500 text-white hover:bg-gray-600".to_string()}
                on_click={Callback::new(move |_| { on_change.run(StepStatus::Pending); })}
            >
                "Reset"
            </Button>
        }.into_any(),
        StepStatus::Pending => view! {
            <Button
                class={"px-3 py-1 text-sm rounded bg-primary text-primary-foreground".to_string()}
                on_click={Callback::new(move |_| { on_change.run(StepStatus::Active); })}
            >
                "Start"
            </Button>
        }.into_any(),
        StepStatus::Active => view! {
            <>
                <Button
                    class={"px-3 py-1 text-sm rounded bg-green-600 text-white".to_string()}
                    on_click={Callback::new(move |_| { on_change.run(StepStatus::Completed); })}
                >
                    "Complete"
                </Button>
                <Button
                    class={"px-3 py-1 text-sm rounded bg-red-600 text-white".to_string()}
                    on_click={Callback::new(move |_| { on_change.run(StepStatus::Failed); })}
                >
                    "Fail"
                </Button>
            </>
        }.into_any(),
        StepStatus::Failed => view! {
            <Button
                class={"px-3 py-1 text-sm rounded bg-gray-500 text-white".to_string()}
                on_click={Callback::new(move |_| { on_change.run(StepStatus::Pending); })}
            >
                "Reset"
            </Button>
        }.into_any(),
        StepStatus::Blocked => view! {
            <Button
                class={"px-3 py-1 text-sm rounded bg-blue-600 text-white".to_string()}
                on_click={Callback::new(move |_| { on_change.run(StepStatus::Active); })}
            >
                "Unblock"
            </Button>
        }.into_any(),
        StepStatus::Skipped => view! {
            <Button
                class={"px-3 py-1 text-sm rounded bg-yellow-600 text-white".to_string()}
                on_click={Callback::new(move |_| { on_change.run(StepStatus::Pending); })}
            >
                "Resume"
            </Button>
        }.into_any(),
        StepStatus::NotApplicable => view! {
            <span class="text-sm text-muted-foreground">"N/A"</span>
        }.into_any(),
    }
}

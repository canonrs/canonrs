use leptos::prelude::*;
use super::{StepStatus, WorkflowStepItem};

/// WorkflowDemo - Ephemeral workflow demonstration
#[component]
pub fn WorkflowDemo() -> impl IntoView {
    let (step1_status, set_step1) = signal(StepStatus::Completed);
    let (step2_status, set_step2) = signal(StepStatus::Active);
    let (step3_status, set_step3) = signal(StepStatus::Blocked);
    let (step4_status, set_step4) = signal(StepStatus::Pending);
    
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
                        <WorkflowStepItem label="Code Review".to_string() status=step1_status />
                    </div>
                    <div class="flex gap-2">
                        {move || match step1_status.get() {
                            StepStatus::Completed => view! {
                                <button class="px-3 py-1 text-sm rounded bg-gray-500 text-white hover:bg-gray-600" on:click=move |_| set_step1.set(StepStatus::Pending)>"Reset"</button>
                            }.into_any(),
                            StepStatus::Pending => view! {
                                <button class="px-3 py-1 text-sm rounded bg-primary text-primary-foreground" on:click=move |_| set_step1.set(StepStatus::Active)>"Start"</button>
                            }.into_any(),
                            StepStatus::Active => view! {
                                <button class="px-3 py-1 text-sm rounded bg-green-600 text-white" on:click=move |_| set_step1.set(StepStatus::Completed)>"Complete"</button>
                                <button class="px-3 py-1 text-sm rounded bg-red-600 text-white" on:click=move |_| set_step1.set(StepStatus::Failed)>"Fail"</button>
                            }.into_any(),
                            StepStatus::Failed => view! {
                                <button class="px-3 py-1 text-sm rounded bg-gray-500 text-white" on:click=move |_| set_step1.set(StepStatus::Pending)>"Reset"</button>
                            }.into_any(),
                            StepStatus::Blocked => view! {
                                <button class="px-3 py-1 text-sm rounded bg-blue-600 text-white" on:click=move |_| set_step1.set(StepStatus::Active)>"Unblock"</button>
                            }.into_any(),
                        }}
                    </div>
                </div>
                
                // Step 2
                <div class="flex items-center gap-4">
                    <div class="flex-1">
                        <WorkflowStepItem label="Approval".to_string() status=step2_status />
                    </div>
                    <div class="flex gap-2">
                        {move || match step2_status.get() {
                            StepStatus::Completed => view! {
                                <button class="px-3 py-1 text-sm rounded bg-gray-500 text-white hover:bg-gray-600" on:click=move |_| set_step2.set(StepStatus::Pending)>"Reset"</button>
                            }.into_any(),
                            StepStatus::Pending => view! {
                                <button class="px-3 py-1 text-sm rounded bg-primary text-primary-foreground" on:click=move |_| set_step2.set(StepStatus::Active)>"Start"</button>
                            }.into_any(),
                            StepStatus::Active => view! {
                                <button class="px-3 py-1 text-sm rounded bg-green-600 text-white" on:click=move |_| set_step2.set(StepStatus::Completed)>"Complete"</button>
                                <button class="px-3 py-1 text-sm rounded bg-red-600 text-white" on:click=move |_| set_step2.set(StepStatus::Failed)>"Fail"</button>
                            }.into_any(),
                            StepStatus::Failed => view! {
                                <button class="px-3 py-1 text-sm rounded bg-gray-500 text-white" on:click=move |_| set_step2.set(StepStatus::Pending)>"Reset"</button>
                            }.into_any(),
                            StepStatus::Blocked => view! {
                                <button class="px-3 py-1 text-sm rounded bg-blue-600 text-white" on:click=move |_| set_step2.set(StepStatus::Active)>"Unblock"</button>
                            }.into_any(),
                        }}
                    </div>
                </div>
                
                // Step 3
                <div class="flex items-center gap-4">
                    <div class="flex-1">
                        <WorkflowStepItem label="Deploy".to_string() status=step3_status />
                    </div>
                    <div class="flex gap-2">
                        {move || match step3_status.get() {
                            StepStatus::Completed => view! {
                                <button class="px-3 py-1 text-sm rounded bg-gray-500 text-white hover:bg-gray-600" on:click=move |_| set_step3.set(StepStatus::Pending)>"Reset"</button>
                            }.into_any(),
                            StepStatus::Pending => view! {
                                <button class="px-3 py-1 text-sm rounded bg-primary text-primary-foreground" on:click=move |_| set_step3.set(StepStatus::Active)>"Start"</button>
                            }.into_any(),
                            StepStatus::Active => view! {
                                <button class="px-3 py-1 text-sm rounded bg-green-600 text-white" on:click=move |_| set_step3.set(StepStatus::Completed)>"Complete"</button>
                                <button class="px-3 py-1 text-sm rounded bg-red-600 text-white" on:click=move |_| set_step3.set(StepStatus::Failed)>"Fail"</button>
                            }.into_any(),
                            StepStatus::Failed => view! {
                                <button class="px-3 py-1 text-sm rounded bg-gray-500 text-white" on:click=move |_| set_step3.set(StepStatus::Pending)>"Reset"</button>
                            }.into_any(),
                            StepStatus::Blocked => view! {
                                <button class="px-3 py-1 text-sm rounded bg-blue-600 text-white" on:click=move |_| set_step3.set(StepStatus::Active)>"Unblock"</button>
                            }.into_any(),
                        }}
                    </div>
                </div>
                
                // Step 4
                <div class="flex items-center gap-4">
                    <div class="flex-1">
                        <WorkflowStepItem label="Monitoring".to_string() status=step4_status />
                    </div>
                    <div class="flex gap-2">
                        {move || match step4_status.get() {
                            StepStatus::Completed => view! {
                                <button class="px-3 py-1 text-sm rounded bg-gray-500 text-white hover:bg-gray-600" on:click=move |_| set_step4.set(StepStatus::Pending)>"Reset"</button>
                            }.into_any(),
                            StepStatus::Pending => view! {
                                <button class="px-3 py-1 text-sm rounded bg-primary text-primary-foreground" on:click=move |_| set_step4.set(StepStatus::Active)>"Start"</button>
                            }.into_any(),
                            StepStatus::Active => view! {
                                <button class="px-3 py-1 text-sm rounded bg-green-600 text-white" on:click=move |_| set_step4.set(StepStatus::Completed)>"Complete"</button>
                                <button class="px-3 py-1 text-sm rounded bg-red-600 text-white" on:click=move |_| set_step4.set(StepStatus::Failed)>"Fail"</button>
                            }.into_any(),
                            StepStatus::Failed => view! {
                                <button class="px-3 py-1 text-sm rounded bg-gray-500 text-white" on:click=move |_| set_step4.set(StepStatus::Pending)>"Reset"</button>
                            }.into_any(),
                            StepStatus::Blocked => view! {
                                <button class="px-3 py-1 text-sm rounded bg-blue-600 text-white" on:click=move |_| set_step4.set(StepStatus::Active)>"Unblock"</button>
                            }.into_any(),
                        }}
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

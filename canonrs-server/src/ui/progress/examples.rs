use leptos::prelude::*;
use super::Progress;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1.5rem; max-width: 600px;">
            <div>
                <p class="text-sm text-gray-600 mb-2">"25% Complete"</p>
                <Progress value=25.0 />
            </div>

            <div>
                <p class="text-sm text-gray-600 mb-2">"50% Complete"</p>
                <Progress value=50.0 />
            </div>

            <div>
                <p class="text-sm text-gray-600 mb-2">"75% Complete"</p>
                <Progress value=75.0 />
            </div>

            <div>
                <p class="text-sm text-gray-600 mb-2">"100% Complete"</p>
                <Progress value=100.0 />
            </div>

            <div>
                <p class="text-sm text-gray-600 mb-2">"Installation in Progress (43%)"</p>
                <Progress value=43.0 />
            </div>
        </div>
    }
}

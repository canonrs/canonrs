use leptos::prelude::*;
use super::LoadingOverlay;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1.5rem;">
            <div>
                <p class="text-sm font-semibold mb-2">"Not Loading"</p>
                <LoadingOverlay loading=false>
                    <div class="p-8 border rounded" style="min-height: 100px;">
                        <p>"Content is visible and interactive"</p>
                    </div>
                </LoadingOverlay>
            </div>

            <div>
                <p class="text-sm font-semibold mb-2">"Loading State"</p>
                <LoadingOverlay loading=true>
                    <div class="p-8 border rounded" style="min-height: 100px;">
                        <p>"Content behind loading overlay"</p>
                    </div>
                </LoadingOverlay>
            </div>
        </div>
    }
}

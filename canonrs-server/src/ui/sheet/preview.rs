use leptos::prelude::*;
use super::sheet_island::SheetIsland;

#[component]
pub fn SheetShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <SheetIsland
                    trigger_label="Open Sheet"
                    title="Sheet Title"
                    description="Sheet slides in from the right. Visibility fully governed via shared state."
                    close_label="Close"
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Sheet visibility and overlay fully governed via shared state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Settings panel"</span>
                <div data-rs-showcase-preview-row="">
                    <SheetIsland
                        trigger_label="Open settings"
                        title="Settings"
                        description="Manage your account settings and preferences."
                        close_label="Close"
                    />
                </div>
            </div>
        </div>
    }
}

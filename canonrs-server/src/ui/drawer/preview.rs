use leptos::prelude::*;
use super::drawer_island::DrawerIsland;

#[component]
pub fn DrawerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DrawerIsland
                    trigger_label="Open Drawer"
                    title="Drawer Title"
                    description="This drawer slides in from the side. State and direction enforced via typed contract."
                    close_label="Close"
                    side="left"
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Drawer direction and visibility enforced via typed contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With content"</span>
                <div data-rs-showcase-preview-row="">
                    <DrawerIsland
                        trigger_label="Open settings"
                        title="Settings"
                        description="Manage your account settings and preferences."
                        close_label="Close"
                        side="right"
                    />
                </div>
            </div>
        </div>
    }
}

use leptos::prelude::*;
use super::drawer_ui::{Drawer, DrawerOverlay, DrawerContent};

#[component]
pub fn DrawerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Drawer>
                    <button type="button" data-rs-drawer-trigger="" data-rs-button="" data-rs-variant="primary">"Open Drawer"</button>
                    <DrawerOverlay />
                    <DrawerContent aria_labelledby="drawer-title-1">
                        <h2 id="drawer-title-1">"Drawer Title"</h2>
                        <p>"This drawer slides in from the side. State and direction enforced via typed contract."</p>
                        <button type="button" data-rs-drawer-close="" data-rs-button="" data-rs-variant="outline">"Close"</button>
                    </DrawerContent>
                </Drawer>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Drawer direction and visibility enforced via typed contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With content"</span>
                <div data-rs-showcase-preview-row="">
                    <Drawer>
                        <button type="button" data-rs-drawer-trigger="" data-rs-button="" data-rs-variant="outline">"Open settings"</button>
                        <DrawerOverlay />
                        <DrawerContent aria_labelledby="drawer-title-2">
                            <h2 id="drawer-title-2">"Settings"</h2>
                            <p>"Manage your account settings and preferences."</p>
                            <button type="button" data-rs-drawer-close="" data-rs-button="" data-rs-variant="outline">"Close"</button>
                        </DrawerContent>
                    </Drawer>
                </div>
            </div>
        </div>
    }
}

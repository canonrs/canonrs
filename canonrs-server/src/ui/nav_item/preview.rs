use leptos::prelude::*;
use super::nav_item_island::NavItemIsland;

#[component]
pub fn NavItemShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                    <NavItemIsland label="Dashboard" href="#" active=true />
                    <NavItemIsland label="Components" href="#" />
                    <NavItemIsland label="Tokens" href="#" />
                    <NavItemIsland label="Settings" href="#" disabled=true />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Active and disabled navigation states enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Inactive"</span>
                <div data-rs-showcase-preview-row="">
                    <NavItemIsland label="Home" href="#" />
                    <NavItemIsland label="About" href="#" />
                    <NavItemIsland label="Contact" href="#" />
                </div>
            </div>
        </div>
    }
}

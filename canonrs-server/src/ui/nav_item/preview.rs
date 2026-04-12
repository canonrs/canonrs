use leptos::prelude::*;
use super::nav_item_boundary::NavItem;

#[component]
pub fn NavItemShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                    <NavItem label="Dashboard" href="#" active=true.into() />
                    <NavItem label="Components" href="#" />
                    <NavItem label="Tokens" href="#" />
                    <NavItem label="Settings" href="#" disabled=true.into() />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Active and disabled navigation states enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Inactive"</span>
                <div data-rs-showcase-preview-row="">
                    <NavItem label="Home" href="#" />
                    <NavItem label="About" href="#" />
                    <NavItem label="Contact" href="#" />
                </div>
            </div>
        </div>
    }
}

use leptos::prelude::*;
use super::nav_item_ui::NavItem;
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn NavItemShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                    <NavItem label="Dashboard".to_string() href="#".to_string() active=ActivityState::Active />
                    <NavItem label="Components".to_string() href="#".to_string() />
                    <NavItem label="Tokens".to_string() href="#".to_string() />
                    <NavItem label="Settings".to_string() href="#".to_string() disabled=DisabledState::Disabled />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Active and disabled navigation states enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Inactive"</span>
                <div data-rs-showcase-preview-row="">
                    <NavItem label="Home".to_string() href="#".to_string() />
                    <NavItem label="About".to_string() href="#".to_string() />
                    <NavItem label="Contact".to_string() href="#".to_string() />
                </div>
            </div>
        </div>
    }
}

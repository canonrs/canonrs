//! @canon-level: ui
//! NavItem - Declarative UI wrapper over NavItemPrimitive

use leptos::prelude::*;
use canonrs_core::primitives::NavItemPrimitive;
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn NavItem(
    #[prop(into)] label: String,
    #[prop(optional, into)] href: Option<String>,
    #[prop(default = false)] active: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] icon: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let active_state = if active { ActivityState::Active } else { ActivityState::Inactive };
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <NavItemPrimitive
            href=href.unwrap_or_else(|| "#".to_string())
            active=active_state
            disabled=disabled_state
            class=class
        >
            {icon.map(|i| view! { <span data-rs-nav-item-icon="">{i()}</span> })}
            <span data-rs-nav-item-label="">{label}</span>
        </NavItemPrimitive>
    }
}

#[component]
pub fn NavItemPreview() -> impl IntoView {
    view! { <NavItem label="Nav Item".to_string() /> }
}

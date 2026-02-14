//! @canon-level: ui
//! NavItem - Declarative UI wrapper over NavItemPrimitive

use leptos::prelude::*;
use crate::primitives::NavItemPrimitive;

#[component]
pub fn NavItem(
    #[prop(into)] label: String,
    #[prop(optional, into)] href: Option<String>,
    #[prop(default = false)] active: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] icon: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <NavItemPrimitive
            href=href.unwrap_or_else(|| "#".to_string())
            active=active
            disabled=disabled
            class=class
            id=id.unwrap_or_default()
        >
            {icon.map(|i| view! { <span data-nav-item-icon="">{i()}</span> })}
            <span data-nav-item-label="">{label}</span>
        </NavItemPrimitive>
    }
}

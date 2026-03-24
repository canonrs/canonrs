//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavItem Primitive - Semantic structural wrapper

use leptos::prelude::*;

#[component]
pub fn NavItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(default = false)] active: bool,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
) -> impl IntoView {
    let aria_current = if active { Some("page") } else { None };
    view! {
        <a
            data-rs-nav-item=""
            data-rs-state={move || if disabled.get() { "disabled" } else if active { "active" } else { "inactive" }}
            aria-current=aria_current
            aria-label=aria_label
            aria-disabled={move || if disabled.get() { "true" } else { "false" }}
            href=href
            class=class
            id=id
        >
            {children.map(|c| c())}
        </a>
    }
}

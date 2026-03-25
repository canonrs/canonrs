//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavItem Primitive - Semantic structural wrapper

use leptos::prelude::*;

#[component]
pub fn NavItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(default = false)] active: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let aria_current = if active { Some("page") } else { None };
    view! {
        <a
            data-rs-nav-item=""
            data-rs-state={if disabled { "disabled" } else if active { "active" } else { "idle" }}
            data-rs-active={if active { Some("true") } else { None }}
            aria-current=aria_current
            aria-label=aria_label
            aria-disabled={if disabled { Some("true") } else { None }}
            href=href
            class={(!class.is_empty()).then(|| class)}
        >
            {children()}
        </a>
    }
}

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
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let data_active = if active { Some("true") } else { None };
    let data_disabled = if disabled { Some("true") } else { None };
    let aria_current = if active { Some("page") } else { None };
    let aria_disabled = if disabled { Some("true") } else { None };
    let resolved_href = if disabled { "#".to_string() } else { href };

    view! {
        <a
        
        
            data-nav-item=""
            attr:data-active=data_active
            attr:data-disabled=data_disabled
            attr:aria-current=aria_current
            attr:aria-disabled=aria_disabled
            attr:aria-label=aria_label
            href=resolved_href
            class=class
            id={if id.as_deref().unwrap_or("").is_empty() { None } else { id }}
        >
            {children.map(|c| c())}
        </a>
    }
}

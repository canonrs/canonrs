//! @canon-level: strict
//! @canon-owner: primitives-team
//! Menu Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn MenuPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <nav
            attr:data-menu=""
            role="menu"
            attr:aria-label={aria_label}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn MenuItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] selected: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-menu-item=""
            attr:data-disabled={disabled.to_string()}
            attr:data-selected={selected.to_string()}
            type="button"
            role="menuitem"
            disabled={disabled}
            tabindex={if disabled { -1 } else { 0 }}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn MenuGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-menu-group=""
            role="group"
            attr:aria-label={aria_label}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

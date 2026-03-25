//! @canon-level: strict
//! @canon-owner: primitives-team
//! Menu Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn MenuPrimitive(
    children: Children,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <nav data-rs-menu="" aria-label=aria_label class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </nav>
    }
}

#[component]
pub fn MenuItemPrimitive(
    children: Children,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-menu-item=""
            data-rs-state={if disabled { "disabled" } else { "default" }}
            data-rs-selected={if selected { Some("true") } else { None }}
            disabled=disabled
            aria-disabled={if disabled { Some("true") } else { None }}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn MenuGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-menu-group="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn MenuLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-menu-label="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn MenuSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-menu-separator="" role="separator" class=class />
    }
}

//! @canon-level: strict
//! @canon-owner: primitives-team
//! Menu Primitive - HTML puro sem ARIA widget

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
            data-menu=""
            attr:aria-label={aria_label.unwrap_or_default()}
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
            data-menu-item=""
            attr:data-disabled={disabled.to_string()}
            attr:data-selected={selected.to_string()}
            type="button"
            disabled={disabled}
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
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-menu-group=""
            class={class}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn MenuLabelPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-menu-label=""
            class={class}
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn MenuSeparatorPrimitive(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-menu-separator=""
            role="separator"
            class={class}
        />
    }
}

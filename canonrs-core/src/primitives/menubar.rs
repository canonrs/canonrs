//! @canon-level: strict
//! @canon-owner: primitives-team
//! Menubar Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn MenubarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menubar=""
            role="menubar"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn MenubarTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = false)] expanded: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-menubar-trigger=""
            role="menuitem"
            aria-haspopup="menu"
            aria-controls={if controls_id.is_empty() { None } else { Some(controls_id) }}
            aria-expanded={expanded.to_string()}
            data-rs-state={if expanded { "open" } else { "closed" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn MenubarContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menubar-content=""
            role="menu"
            id={if content_id.is_empty() { None } else { Some(content_id) }}
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn MenubarSubItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-menubar-subitem=""
            role="menuitem"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn MenubarSeparatorPrimitive(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-menubar-separator=""
            role="separator"
            class=class
        />
    }
}

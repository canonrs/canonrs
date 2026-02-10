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
            data-menubar=""
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
    let controls_id_clone = controls_id.clone();
    view! {
        <button
            data-menubar-trigger=""
            data-target={controls_id}
            type="button"
            role="menuitem"
            attr:aria-haspopup="menu"
            attr:aria-controls={controls_id_clone}
            attr:aria-expanded={expanded.to_string()}
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
    let content_id_clone = content_id.clone();
    view! {
        <div
            data-menubar-content=""
            data-menu={content_id}
            role="menu"
            id=content_id_clone
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
            data-menubar-subitem=""
            type="button"
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
            data-menubar-separator=""
            role="separator"
            class=class
        />
    }
}

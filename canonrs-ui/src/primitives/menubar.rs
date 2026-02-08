//! @canon-level: strict
//! @canon-owner: primitives-team
//! Menubar Primitive - HTML puro + ARIA
//! Base: Navigation com roving tabindex

use leptos::prelude::*;

#[component]
pub fn MenubarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-menubar=""
            role="menubar"
            attr:aria-orientation="horizontal"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn MenubarItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = -1)] tabindex: i32,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            attr:data-menubar-item=""
            role="menuitem"
            tabindex={tabindex}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn MenubarTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = -1)] tabindex: i32,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = false)] expanded: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-menubar-trigger=""
            type="button"
            role="menuitem"
            tabindex={tabindex}
            attr:aria-haspopup="menu"
            attr:aria-controls={controls_id}
            attr:aria-expanded={if expanded { "true" } else { "false" }}
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
            role="menu"
            id={content_id}
            attr:data-menubar-content=""
            tabindex="-1"
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
            attr:data-menubar-subitem=""
            type="button"
            role="menuitem"
            tabindex="-1"
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
            attr:data-menubar-separator=""
            role="separator"
            class=class
        />
    }
}

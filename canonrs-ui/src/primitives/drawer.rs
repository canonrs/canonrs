//! @canon-level: strict
//! @canon-owner: primitives-team
//! Drawer Primitive - Persistent/Modal sidebar drawer

use leptos::prelude::*;

#[component]
pub fn DrawerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] mode: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <aside
            data-drawer=""
            attr:data-mode={(!mode.is_empty()).then(|| mode)}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </aside>
    }
}

#[component]
pub fn DrawerTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-drawer-trigger=""
            type="button"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn DrawerContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-drawer-content=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DrawerOverlayPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-drawer-overlay=""
            attr:aria-hidden="true"
            class={class}
            id={id}
        />
    }
}

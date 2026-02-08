use leptos::prelude::*;
use crate::primitives::{
    ContextMenuPrimitive,
    ContextMenuTriggerPrimitive,
    ContextMenuContentPrimitive,
    ContextMenuItemPrimitive,
    ContextMenuSeparatorPrimitive,
};

#[component]
pub fn ContextMenu(
    children: Children,
    open: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ContextMenuPrimitive
            open=open
            class=class
            id=id
        >
            {children()}
        </ContextMenuPrimitive>
    }
}

#[component]
pub fn ContextMenuTrigger(
    children: Children,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ContextMenuTriggerPrimitive
            controls_id=controls_id
            class=class
            id=id
        >
            {children()}
        </ContextMenuTriggerPrimitive>
    }
}

#[component]
pub fn ContextMenuContent(
    children: Children,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ContextMenuContentPrimitive
            content_id=content_id
            class=class
        >
            {children()}
        </ContextMenuContentPrimitive>
    }
}

#[component]
pub fn ContextMenuItem(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ContextMenuItemPrimitive
            class=class
            id=id
        >
            {children()}
        </ContextMenuItemPrimitive>
    }
}

#[component]
pub fn ContextMenuSeparator(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ContextMenuSeparatorPrimitive
            class=class
            id=id
        />
    }
}

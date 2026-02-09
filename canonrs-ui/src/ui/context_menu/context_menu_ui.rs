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
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ContextMenuPrimitive
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
    #[prop(into)] target_context_menu_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ContextMenuTriggerPrimitive
            target_context_menu_id=target_context_menu_id
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
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ContextMenuContentPrimitive
            class=class
            id=id
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

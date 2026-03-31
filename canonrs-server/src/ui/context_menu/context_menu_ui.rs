
use leptos::prelude::*;
use canonrs_core::meta::VisibilityState;
use canonrs_core::primitives::{
    ContextMenuPrimitive, ContextMenuTriggerPrimitive, ContextMenuContentPrimitive,
    ContextMenuItemPrimitive, ContextMenuSeparatorPrimitive,
};

#[component]
pub fn ContextMenu(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ContextMenuPrimitive class=class>
            {children()}
        </ContextMenuPrimitive>
    }
}

#[component]
pub fn ContextMenuTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ContextMenuTriggerPrimitive class=class>
            {children()}
        </ContextMenuTriggerPrimitive>
    }
}

#[component]
pub fn ContextMenuContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ContextMenuContentPrimitive state=VisibilityState::Closed class=class>
            {children()}
        </ContextMenuContentPrimitive>
    }
}

#[component]
pub fn ContextMenuItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ContextMenuItemPrimitive class=class>
            {children()}
        </ContextMenuItemPrimitive>
    }
}

#[component]
pub fn ContextMenuSeparator(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ContextMenuSeparatorPrimitive class=class />
    }
}

#[component]
pub fn ContextMenuPreview() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger>
                <span>"Right-click"</span>
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuItem>"Item"</ContextMenuItem>
            </ContextMenuContent>
        </ContextMenu>
    }
}

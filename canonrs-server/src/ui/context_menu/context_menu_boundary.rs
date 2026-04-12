//! @canon-level: strict
//! ContextMenu Island — passthrough only

use leptos::prelude::*;
use super::context_menu_ui::{
    ContextMenu as ContextMenuUi,
    ContextMenuTrigger as ContextMenuTriggerUi,
    ContextMenuContent as ContextMenuContentUi,
    ContextMenuItem as ContextMenuItemUi
};

#[component]
pub fn ContextMenu(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ContextMenuUi class=class.unwrap_or_default()>{children()}</ContextMenuUi> }
}

#[component]
pub fn ContextMenuTrigger(children: Children) -> impl IntoView {
    view! { <ContextMenuTriggerUi>{children()}</ContextMenuTriggerUi> }
}

#[component]
pub fn ContextMenuContent(children: Children) -> impl IntoView {
    view! { <ContextMenuContentUi>{children()}</ContextMenuContentUi> }
}

#[component]
pub fn ContextMenuItem(children: Children) -> impl IntoView {
    view! { <ContextMenuItemUi>{children()}</ContextMenuItemUi> }
}

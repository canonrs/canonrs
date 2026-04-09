//! @canon-level: strict
//! ContextMenu Island — passthrough only

use leptos::prelude::*;
use super::context_menu_ui::{ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem};

#[component]
pub fn ContextMenuIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ContextMenu class=class.unwrap_or_default()>{children()}</ContextMenu> }
}

#[component]
pub fn ContextMenuTriggerIsland(children: Children) -> impl IntoView {
    view! { <ContextMenuTrigger>{children()}</ContextMenuTrigger> }
}

#[component]
pub fn ContextMenuContentIsland(children: Children) -> impl IntoView {
    view! { <ContextMenuContent>{children()}</ContextMenuContent> }
}

#[component]
pub fn ContextMenuItemIsland(children: Children) -> impl IntoView {
    view! { <ContextMenuItem>{children()}</ContextMenuItem> }
}

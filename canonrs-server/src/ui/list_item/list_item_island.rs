//! @canon-level: strict
//! ListItem Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::list_item_ui::{List, ListItem, ListItemTitle, ListItemDescription};



#[component]
pub fn ListIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <List class=class.unwrap_or_default()>{children()}</List>
    }
}

#[component]
pub fn ListItemIsland(
    children: Children,
    #[prop(optional)] selectable: Option<bool>,
    #[prop(optional)] selected: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ListItem
            selectable=selectable.unwrap_or(false)
            selected=selected.unwrap_or(false)
            disabled=disabled.unwrap_or(false)
            class=class.unwrap_or_default()
        >
            {children()}
        </ListItem>
    }
}

#[component]
pub fn ListItemTitleIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ListItemTitle class=class.unwrap_or_default()>{children()}</ListItemTitle> }
}

#[component]
pub fn ListItemDescriptionIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ListItemDescription class=class.unwrap_or_default()>{children()}</ListItemDescription> }
}

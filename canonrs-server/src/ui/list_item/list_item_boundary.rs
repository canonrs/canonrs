//! @canon-level: strict
//! ListItem Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::list_item_ui::{
    List as ListUi,
    ListItem as ListItemUi,
    ListItemTitle as ListItemTitleUi,
    ListItemDescription as ListItemDescriptionUi
};



#[component]
pub fn List(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ListUi class=class.unwrap_or_default()>{children()}</ListUi>
    }
}

#[component]
pub fn ListItem(
    children: Children,
    #[prop(optional)] selectable: Option<bool>,
    #[prop(optional)] selected: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ListItemUi
            selectable=selectable.unwrap_or(false)
            selected=selected.unwrap_or(false)
            disabled=disabled.unwrap_or(false)
            class=class.unwrap_or_default()
        >
            {children()}
        </ListItemUi>
    }
}

#[component]
pub fn ListItemTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ListItemTitleUi class=class.unwrap_or_default()>{children()}</ListItemTitleUi> }
}

#[component]
pub fn ListItemDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <ListItemDescriptionUi class=class.unwrap_or_default()>{children()}</ListItemDescriptionUi> }
}

//! @canon-level: strict
//! ListItem Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::list_item_ui::{
    List as ListUi,
    ListItem as ListItemUi,
    ListItemTitle as ListItemTitleUi,
    ListItemDescription as ListItemDescriptionUi,
};
pub use super::list_item_ui::ListSelectionMode;

#[component]
pub fn List(
    children: Children,
    #[prop(default = ListSelectionMode::None)] selection_mode: ListSelectionMode,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListUi selection_mode=selection_mode class=class>
            {children()}
        </ListUi>
    }
}

#[component]
pub fn ListItem(
    children: Children,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListItemUi selected=selected disabled=disabled class=class>
            {children()}
        </ListItemUi>
    }
}

#[component]
pub fn ListItemTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListItemTitleUi class=class>
            {children()}
        </ListItemTitleUi>
    }
}

#[component]
pub fn ListItemDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListItemDescriptionUi class=class>
            {children()}
        </ListItemDescriptionUi>
    }
}

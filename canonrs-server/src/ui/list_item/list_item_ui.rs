#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{
    ListPrimitive, ListItemPrimitive,
    ListItemTitlePrimitive, ListItemDescriptionPrimitive
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ListSelectionMode { None, Single, Multiple }

#[component]
pub fn List(
    children: Children,
    #[prop(default = ListSelectionMode::None)] _selection_mode: ListSelectionMode,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListPrimitive class={class}>
            {children()}
        </ListPrimitive>
    }
}

#[component]
pub fn ListItem(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] selectable: bool,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <ListItemPrimitive class={class}>
            <div
                data-rs-list-item-content=""
                data-rs-selectable={selectable.then_some("")}
                data-rs-selected={selected.then_some("")}
                data-rs-disabled={disabled.then_some("")}
                tabindex={if selectable && !disabled { Some("0") } else { None }}
                aria-selected={if selectable { Some(selected.to_string()) } else { None }}
                aria-disabled={if disabled { Some("true") } else { None }}
            >
                {children()}
            </div>
        </ListItemPrimitive>
    }
}

#[component]
pub fn ListItemTitle(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListItemTitlePrimitive class={class}>
            {children()}
        </ListItemTitlePrimitive>
    }
}

#[component]
pub fn ListItemDescription(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListItemDescriptionPrimitive class={class}>
            {children()}
        </ListItemDescriptionPrimitive>
    }
}

#[component]
pub fn ListItemPreview() -> impl IntoView {
    view! { <List>"Item"</List> }
}

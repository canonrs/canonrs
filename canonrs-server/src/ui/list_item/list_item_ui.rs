#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{
    ListPrimitive, ListItemPrimitive,
    ListItemTitlePrimitive, ListItemDescriptionPrimitive
};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ListSelectionMode { #[default] None, Single, Multiple }

impl ListSelectionMode {
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::None     => None,
            Self::Single   => Some("single"),
            Self::Multiple => Some("multiple"),
        }
    }
}

#[component]
pub fn List(
    children: Children,
    #[prop(default = ListSelectionMode::None)] selection_mode: ListSelectionMode,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListPrimitive
            class=class
            attr:data-rs-selection=selection_mode.as_str()
        >
            {children()}
        </ListPrimitive>
    }
}

#[component]
pub fn ListItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <ListItemPrimitive
            class=class
            attr:data-rs-state=if selected { "selected" } else { "idle" }
            attr:data-rs-disabled=disabled.then_some("")
            attr:aria-selected=selected.then_some("true")
            attr:aria-disabled=disabled.then_some("true")
            attr:tabindex=if !disabled { Some("0") } else { None }
        >
            <div data-rs-list-item-content="">
                {children()}
            </div>
        </ListItemPrimitive>
    }
}

#[component]
pub fn ListItemTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListItemTitlePrimitive class=class>
            {children()}
        </ListItemTitlePrimitive>
    }
}

#[component]
pub fn ListItemDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListItemDescriptionPrimitive class=class>
            {children()}
        </ListItemDescriptionPrimitive>
    }
}

#[component]
pub fn ListItemPreview() -> impl IntoView {
    view! { <List selection_mode=ListSelectionMode::Single>"Item"</List> }
}

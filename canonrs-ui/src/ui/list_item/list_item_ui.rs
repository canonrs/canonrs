use leptos::prelude::*;
use crate::primitives::{ListPrimitive, ListItemPrimitive, ListItemTitlePrimitive, ListItemDescriptionPrimitive};

#[component]
pub fn List(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ListPrimitive
            class={class}
            id={id.unwrap_or_default()}
        >
            {children.map(|c| c())}
        </ListPrimitive>
    }
}

#[component]
pub fn ListItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
    #[prop(default = false)] selectable: bool,
    #[prop(default = false)] selected: bool,
) -> impl IntoView {
    view! {
        <ListItemPrimitive
            class={class}
            id={id.unwrap_or_default()}
        >
            <div
                data-list-item-content=""
                data-selectable={selectable.then_some("")}
                data-selected={selected.then_some("")}
                tabindex={if selectable { Some("0") } else { None }}
                aria-selected={if selectable { Some(selected.to_string()) } else { None }}
            >
                {children.map(|c| c())}
            </div>
        </ListItemPrimitive>
    }
}

#[component]
pub fn ListItemTitle(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListItemTitlePrimitive class={class}>
            {children.map(|c| c())}
        </ListItemTitlePrimitive>
    }
}

#[component]
pub fn ListItemDescription(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ListItemDescriptionPrimitive class={class}>
            {children.map(|c| c())}
        </ListItemDescriptionPrimitive>
    }
}

//! # List Block
use leptos::prelude::*;

#[component]
pub fn List(
    #[prop(default = false)] ordered: bool,
    #[prop(default = false)] interactive: bool,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    if ordered {
        view! {
            <ol class=format!("canon-list canon-list--ordered {} {}", if interactive { "canon-list--interactive" } else { "" }, class)
                data-block="list" data-block-version="1">
                {children()}
            </ol>
        }.into_any()
    } else {
        view! {
            <ul class=format!("canon-list {} {}", if interactive { "canon-list--interactive" } else { "" }, class)
                data-block="list" data-block-version="1">
                {children()}
            </ul>
        }.into_any()
    }
}

#[component]
pub fn ListItem(
    #[prop(default = false)] selected: bool,
    #[prop(optional, into)] item_id: Option<String>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            class=format!("canon-list__item {} {}", if selected { "canon-list__item--selected" } else { "" }, class)
            data-action="click"
            data-list-item-id=item_id
        >
            {children()}
        </li>
    }
}

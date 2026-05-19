//! TocItem — shared type for Table of Contents primitives

use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct TocItem {
    pub id: String,
    pub text: String,
    pub level: u8,
}

impl TocItem {
    pub fn new(id: String, text: String, level: u8) -> Self {
        Self { id, text, level }
    }
}

#[component]
pub fn TocItemRowPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <div data-rs-toc-item-row="" class=class>{children()}</div> }
}

#[component]
pub fn TocExpandIconPrimitive() -> impl IntoView {
    view! { <span data-rs-toc-expand-icon=""/> }
}

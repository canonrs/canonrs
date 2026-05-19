#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{ScrollAreaPrimitive, ScrollOrientation};

#[component]
pub fn ScrollArea(
    children: Children,
    #[prop(default = ScrollOrientation::Vertical)] orientation: ScrollOrientation,
    #[prop(default = true)] auto_hide: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] style: String,
    #[prop(into, optional)] viewport_id: Option<String>,
) -> impl IntoView {
    view! {
        <ScrollAreaPrimitive
            orientation=orientation
            auto_hide=auto_hide
            class=class
            style=style
            viewport_id=viewport_id.unwrap_or_default()
        >
            {children()}
        </ScrollAreaPrimitive>
    }
}


#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{ScrollAreaPrimitive, ScrollOrientation};

#[component]
pub fn ScrollArea(
    children: Children,
    #[prop(default = ScrollOrientation::Vertical)] orientation: ScrollOrientation,
    #[prop(default = true)] auto_hide: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] viewport_id: Option<String>,
) -> impl IntoView {
    view! {
        <ScrollAreaPrimitive
            orientation=orientation
            auto_hide=auto_hide
            class=class
            viewport_id=viewport_id.unwrap_or_default()
        >
            {children()}
        </ScrollAreaPrimitive>
    }
}

#[component]
pub fn ScrollAreaPreview() -> impl IntoView {
    view! {
        <div style="height:200px;">
            <ScrollArea>
                <div style="padding:1rem;">
                    {(1..=20).map(|i| view! {
                        <p style="margin:0;padding:4px 0;">{format!("Item {:02}", i)}</p>
                    }).collect::<Vec<_>>()}
                </div>
            </ScrollArea>
        </div>
    }
}

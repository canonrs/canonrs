//! @canon-level: ui
//! ScrollArea UI - Enterprise scroll com custom scrollbar

use leptos::prelude::*;
use super::scroll_area_primitive::{ScrollAreaPrimitive, ScrollOrientation};

#[component]
pub fn ScrollArea(
    children: Children,
    #[prop(default = ScrollOrientation::Vertical)] orientation: ScrollOrientation,
    #[prop(default = true)] auto_hide: bool,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <ScrollAreaPrimitive
            orientation=orientation
            auto_hide=auto_hide
            class={class.unwrap_or_default()}
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

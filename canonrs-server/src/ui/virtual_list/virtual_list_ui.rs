#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{VirtualListPrimitive, VirtualListViewportPrimitive, ScrollOrientation};
use crate::ui::scroll_area::scroll_area_boundary::ScrollArea;

#[component]
pub fn VirtualList(
    #[prop(into)] items_count: usize,
    #[prop(into, default = 40.0f64)] item_height: f64,
    #[prop(into, default = 400u32)] height: u32,
    #[prop(into, default = String::new())] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <ScrollArea
            orientation=ScrollOrientation::Vertical
            attr:style=format!("height:{}px;", height)
        >
            <VirtualListPrimitive
                class=class
                attr:data-items-count=items_count.to_string()
                attr:data-item-height=item_height.to_string()
            >
                <VirtualListViewportPrimitive>
                    {children()}
                </VirtualListViewportPrimitive>
            </VirtualListPrimitive>
        </ScrollArea>
    }
}

#[component]
pub fn VirtualListPreview() -> leptos::prelude::AnyView {
    view! { <VirtualList items_count=3usize item_height=40.0f64>"Item"</VirtualList> }.into_any()
}

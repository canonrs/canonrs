//! ScrollArea UI - Enterprise scroll com custom scrollbar

use leptos::prelude::*;
use super::scroll_area_primitive::{ScrollAreaPrimitive, ScrollOrientation};

#[component]
pub fn ScrollArea(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = ScrollOrientation::Vertical)] orientation: ScrollOrientation,
    #[prop(default = true)] auto_hide: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <ScrollAreaPrimitive
            id={id}
            class={class}
            orientation={orientation}
            auto_hide={auto_hide}
        >
            {children.map(|c| c())}
        </ScrollAreaPrimitive>
    }
}

//! @canon-level: strict
//! VirtualList Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::virtual_list_ui::VirtualList;

#[island]
pub fn VirtualListInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::virtual_list::init_all();
        });
    }
    view! { <></> }
}

#[component]
pub fn VirtualListIsland(
    #[prop(into)] items_count: usize,
    #[prop(into, default = 40.0f64)] item_height: f64,
    children: Children,
) -> impl IntoView {
    view! {
        <VirtualListInit />
        <VirtualList items_count=items_count item_height=item_height>
            {children()}
        </VirtualList>
    }
}

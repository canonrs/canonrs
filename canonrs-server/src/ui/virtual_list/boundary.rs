//! @canon-level: strict
//! VirtualList Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::virtual_list_ui::VirtualList as VirtualListUi;



#[component]
pub fn VirtualList(
    #[prop(into)] items_count: usize,
    #[prop(into, default = 40.0f64)] item_height: f64,
    children: Children,
) -> impl IntoView {
    view! {
        <VirtualListUi items_count=items_count item_height=item_height>
            {children()
};
        </VirtualListUi>
    }
}

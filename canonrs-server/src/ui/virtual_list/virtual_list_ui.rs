
use leptos::prelude::*;

#[component]
pub fn VirtualList(
    #[prop(into)] items_count: usize,
    #[prop(into)] item_height: f64,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-rs-virtual-list=""
            data-items-count=items_count.to_string()
            data-item-height=item_height.to_string()
            class="virtual-list"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn VirtualListPreview() -> leptos::prelude::AnyView {
    view! { <VirtualList items_count=3usize item_height=40.0f64>"Item"</VirtualList> }.into_any()
}

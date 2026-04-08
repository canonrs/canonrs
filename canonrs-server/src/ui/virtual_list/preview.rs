use leptos::prelude::*;
use super::virtual_list_island::VirtualListIsland;

#[component]
pub fn VirtualListShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <VirtualListIsland items_count=10000usize item_height=40.0f64>
                    ""
                </VirtualListIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "List virtualization enforced structurally for large datasets."
            </p>
        </div>
    }
}

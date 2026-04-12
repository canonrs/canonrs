use leptos::prelude::*;
use super::boundary::VirtualList;

#[component]
pub fn VirtualListShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <VirtualList items_count=10000usize item_height=40.0f64>
                    ""
                </VirtualList>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "List virtualization enforced structurally for large datasets."
            </p>
        </div>
    }
}

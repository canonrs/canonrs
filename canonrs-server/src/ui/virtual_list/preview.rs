use leptos::prelude::*;
use super::virtual_list_boundary::VirtualList;
use canonrs_core::primitives::layout::container::{ContainerPrimitive as Container, ContainerSize};

#[component]
pub fn VirtualListShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Container size=ContainerSize::Sm>
                    <VirtualList items_count=10000usize item_height=40.0f64>
                        ""
                    </VirtualList>
                </Container>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "List virtualization enforced structurally for large datasets."
            </p>
        </div>
    }
}

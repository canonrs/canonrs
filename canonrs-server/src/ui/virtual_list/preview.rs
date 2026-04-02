use leptos::prelude::*;
use super::virtual_list_ui::VirtualList;
use crate::ui::scroll_area::ScrollArea;

#[component]
pub fn VirtualListShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="height:400px;">
                    <ScrollArea>
                        <VirtualList items_count=10000usize item_height=40.0>
                            <div data-rs-virtual-list-inner="">
                                {(1..=10000usize).map(|i| view! {
                                    <div data-rs-virtual-list-item="" style="height:40px;display:flex;align-items:center;gap:var(--space-sm);padding:0 var(--space-md);border-bottom:1px solid var(--border);">
                                        <span style="opacity:0.4;font-size:var(--font-size-xs);">{format!("#{:05}", i)}</span>
                                        <span>{format!("Item {}", i)}</span>
                                    </div>
                                }).collect_view()}
                            </div>
                        </VirtualList>
                    </ScrollArea>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "List virtualization enforced structurally for large datasets."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"10,000 items — only visible rows rendered"</span>
                <div data-rs-showcase-preview-row="">
                    <div style="height:400px;">
                        <ScrollArea>
                            <VirtualList items_count=10000usize item_height=48.0>
                                <div data-rs-virtual-list-inner="">
                                    {(1..=10000usize).map(|i| view! {
                                        <div data-rs-virtual-list-item="" style="height:48px;display:flex;align-items:center;gap:var(--space-md);padding:0 var(--space-md);border-bottom:1px solid var(--border);">
                                            <span style="opacity:0.4;font-size:var(--font-size-xs);">{format!("#{:05}", i)}</span>
                                            <span>{format!("Row item {}", i)}</span>
                                        </div>
                                    }).collect_view()}
                                </div>
                            </VirtualList>
                        </ScrollArea>
                    </div>
                </div>
            </div>
        </div>
    }
}

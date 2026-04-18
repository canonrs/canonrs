use canonrs_core::primitives::ScrollOrientation;
use leptos::prelude::*;
use super::scroll_area_boundary::ScrollArea;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ScrollAreaShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Custom scrollbar with auto-hide and orientation control."
            </p>
            <ScrollArea style="height:200px;">
                <div style="padding:var(--space-md);display:flex;flex-direction:column;gap:var(--space-xs);">
                    {(1..=20).map(|i| view! {
                        <div style="padding:var(--space-xs) 0;border-bottom:1px solid rgba(255,255,255,0.05);">
                            {format!("Item {:02}", i)}
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </ScrollArea>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Horizontal"</span>
                <div style="width:100%;overflow:hidden;">
                    <ScrollArea orientation=ScrollOrientation::Horizontal>
                        <div style="display:flex;gap:var(--space-md);padding:var(--space-sm);width:200px;">
                            {(1..=12).map(|i| view! {
                                <div style="width:120px;flex-shrink:0;padding:var(--space-sm);border:1px solid rgba(255,255,255,0.08);border-radius:var(--radius-md);text-align:center;">
                                    {format!("Card {:02}", i)}
                                </div>
                            }).collect::<Vec<_>>()}
                        </div>
                    </ScrollArea>
                </div>
            </Stack>
        </Stack>
    }
}

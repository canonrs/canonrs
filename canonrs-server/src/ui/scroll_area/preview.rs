use canonrs_core::primitives::ScrollOrientation;
use leptos::prelude::*;
use super::scroll_area_island::ScrollAreaIsland;

#[component]
pub fn ScrollAreaShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="height:200px;width:100%;display:flex;">
                    <ScrollAreaIsland>
                        <div style="padding:var(--space-md);display:flex;flex-direction:column;gap:var(--space-xs);">
                            {(1..=20).map(|i| view! {
                                <div style="padding:var(--space-xs) 0;border-bottom:1px solid rgba(255,255,255,0.05);">
                                    {format!("Item {:02}", i)}
                                </div>
                            }).collect::<Vec<_>>()}
                        </div>
                    </ScrollAreaIsland>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Custom scrollbar with auto-hide and orientation control."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Horizontal"</span>
                <div data-rs-showcase-preview-row="">
                    <div style="height:72px;width:320px;display:flex;">
                        <ScrollAreaIsland orientation=ScrollOrientation::Horizontal>
                            <div style="display:flex;gap:var(--space-md);padding:var(--space-sm);">
                                {(1..=12).map(|i| view! {
                                    <div style="width:80px;flex-shrink:0;padding:var(--space-xs);border:1px solid rgba(255,255,255,0.08);border-radius:var(--radius-md);text-align:center;white-space:nowrap;">
                                        {format!("Card {:02}", i)}
                                    </div>
                                }).collect::<Vec<_>>()}
                            </div>
                        </ScrollAreaIsland>
                    </div>
                </div>
            </div>
        </div>
    }
}

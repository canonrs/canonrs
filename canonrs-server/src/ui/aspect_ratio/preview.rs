use leptos::prelude::*;
use super::aspect_ratio_island::AspectRatioIsland;

#[component]
pub fn AspectRatioShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;max-width:480px;margin:0 auto;">
                <AspectRatioIsland ratio_w=16.0f32 ratio_h=9.0f32>
                    <div style="width:100%;height:100%;background:var(--theme-surface-muted);border-radius:var(--radius-md);display:flex;align-items:center;justify-content:center;">
                        "16 / 9"
                    </div>
                </AspectRatioIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Aspect ratio enforced structurally with no layout drift."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Ratios"</span>
                <div data-rs-showcase-preview-row="" style="display:grid;grid-template-columns:repeat(3,1fr);gap:var(--space-md);width:100%;">
                    <AspectRatioIsland ratio_w=4.0f32 ratio_h=3.0f32>
                        <div style="width:100%;height:100%;background:var(--theme-surface-muted);border-radius:var(--radius-md);display:flex;align-items:center;justify-content:center;">
                            "4 / 3"
                        </div>
                    </AspectRatioIsland>
                    <AspectRatioIsland ratio_w=1.0f32 ratio_h=1.0f32>
                        <div style="width:100%;height:100%;background:var(--theme-surface-muted);border-radius:var(--radius-md);display:flex;align-items:center;justify-content:center;">
                            "1 / 1"
                        </div>
                    </AspectRatioIsland>
                    <AspectRatioIsland ratio_w=21.0f32 ratio_h=9.0f32>
                        <div style="width:100%;height:100%;background:var(--theme-surface-muted);border-radius:var(--radius-md);display:flex;align-items:center;justify-content:center;">
                            "21 / 9"
                        </div>
                    </AspectRatioIsland>
                </div>
            </div>
        </div>
    }
}

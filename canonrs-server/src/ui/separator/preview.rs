use leptos::prelude::*;
use super::separator_ui::Separator;
use canonrs_core::Orientation;

#[component]
pub fn SeparatorShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;">
                <div style="display:flex;flex-direction:column;gap:var(--space-md);">
                    <span>"Above"</span>
                    <Separator orientation=Orientation::Horizontal />
                    <span>"Below"</span>
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Separator semantics enforced via orientation and role contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Vertical"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;align-items:center;gap:var(--space-md);height:40px;">
                    <span>"Left"</span>
                    <Separator orientation=Orientation::Vertical />
                    <span>"Center"</span>
                    <Separator orientation=Orientation::Vertical />
                    <span>"Right"</span>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Semantic"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-md);">
                    <span>"Section A"</span>
                    <Separator decorative=false aria_label="Between sections" />
                    <span>"Section B"</span>
                </div>
            </div>
        </div>
    }
}

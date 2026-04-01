use leptos::prelude::*;
use super::textarea_ui::Textarea;

#[component]
pub fn TextareaShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Textarea placeholder="Type here..." rows=3 />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "All form states mapped directly to DOM and ARIA."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Rows"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Textarea placeholder="2 rows" rows=2 />
                    <Textarea placeholder="5 rows" rows=5 />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Textarea placeholder="Readonly" readonly=true rows=2 />
                    <Textarea placeholder="Disabled" disabled=true rows=2 />
                </div>
            </div>
        </div>
    }
}

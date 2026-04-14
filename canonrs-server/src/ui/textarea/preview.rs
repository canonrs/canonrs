use super::textarea_boundary::Textarea;
use leptos::prelude::*;
use canonrs_core::meta::DisabledState;

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
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-preview-dev-grid="">
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Default"</span>
                        <Textarea placeholder="Default" rows=2 />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Success"</span>
                        <Textarea placeholder="Success" rows=2 />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Warning"</span>
                        <Textarea placeholder="Warning" rows=2 />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Error"</span>
                        <Textarea placeholder="Error" rows=2 />
                    </div>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Rows"</span>
                <div data-rs-preview-dev-grid="">
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"2 rows"</span>
                        <Textarea placeholder="2 rows" rows=2 />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"5 rows"</span>
                        <Textarea placeholder="5 rows" rows=5 />
                    </div>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-preview-dev-grid="">
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Readonly"</span>
                        <Textarea placeholder="Readonly" readonly=true rows=2 />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Disabled"</span>
                        <Textarea placeholder="Disabled" disabled=DisabledState::Disabled rows=2 />
                    </div>
                </div>
            </div>

        </div>
    }
}

use leptos::prelude::*;
use super::copy_button_boundary::CopyButton;

#[component]
pub fn CopyButtonShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <CopyButton
                    text="cargo add canonrs"
                    aria_label="Copy to clipboard"
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Copy state lifecycle fully encoded in DOM state machine."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States (idle → copied → reset)"</span>
                <div data-rs-showcase-preview-row="">
                    <CopyButton
                        text="npm install react"
                        aria_label="Copy npm"
                    />
                    <CopyButton
                        text="cargo add leptos"
                        aria_label="Copy cargo"
                    />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Error state (no text)"</span>
                <div data-rs-showcase-preview-row="">
                    <CopyButton
                        aria_label="Copy empty"
                    />
                </div>
            </div>

        </div>
    }
}

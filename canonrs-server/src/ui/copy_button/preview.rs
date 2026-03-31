use leptos::prelude::*;
use super::copy_button_ui::CopyButton;

#[component]
pub fn CopyButtonShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <CopyButton
                    id="preview-copy".to_string()
                    text="cargo add canonrs".to_string()
                    aria_label="Copy to clipboard".to_string()
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Copy state lifecycle fully encoded in DOM state machine."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States (idle → copied → reset)"</span>
                <div data-rs-showcase-preview-row="">
                    <CopyButton
                        id="preview-copy-1".to_string()
                        text="npm install react".to_string()
                        aria_label="Copy npm".to_string()
                    />
                    <CopyButton
                        id="preview-copy-2".to_string()
                        text="cargo add leptos".to_string()
                        aria_label="Copy cargo".to_string()
                    />
                </div>
            </div>
        </div>
    }
}

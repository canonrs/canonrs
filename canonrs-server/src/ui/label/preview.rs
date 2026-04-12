use crate::ui::input::boundary::Input;
use leptos::prelude::*;
use super::label_ui::Label;

#[component]
pub fn LabelShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                    <Label for_id="label-input">"Username"</Label>
                    <Input placeholder="johndoe" />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Label-to-input association enforced via explicit html_for contract."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Multiple labels"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                        <Label for_id="label-email"><span>{"Email"}</span></Label>
                        <Input placeholder="john@example.com" input_type="email" />
                    </div>
                    <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                        <Label for_id="label-password"><span>{"Password"}</span></Label>
                        <Input placeholder="••••••••" input_type="password" />
                    </div>
                </div>
            </div>

        </div>
    }
}

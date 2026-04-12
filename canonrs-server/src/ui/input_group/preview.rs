use leptos::prelude::*;
use super::input_group_boundary::InputGroup;
use crate::ui::input::Input;

#[component]
pub fn InputGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <InputGroup merge_radius=true>
                    <span data-rs-input-group-addon="">"@"</span>
                    <Input placeholder="username" name="username" />
                </InputGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Grouped inputs maintain consistent structure and visual merging."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Detached"</span>
                <div data-rs-showcase-preview-row="">
                    <InputGroup>
                        <span data-rs-input-group-addon="">"@"</span>
                        <Input placeholder="username" name="username-detached" />
                    </InputGroup>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"URL input"</span>
                <div data-rs-showcase-preview-row="">
                    <InputGroup merge_radius=true>
                        <span data-rs-input-group-addon="">"https://"</span>
                        <Input placeholder="example.com" name="url" />
                    </InputGroup>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With suffix"</span>
                <div data-rs-showcase-preview-row="">
                    <InputGroup merge_radius=true>
                        <Input placeholder="0.00" name="amount" />
                        <span data-rs-input-group-addon="">"USD"</span>
                    </InputGroup>
                </div>
            </div>
        </div>
    }
}

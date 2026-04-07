use leptos::prelude::*;
use super::input_group_island::InputGroupIsland;
use crate::ui::input::input_island::InputIsland;

#[component]
pub fn InputGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <InputGroupIsland merge_radius=true>
                    <span data-rs-input-group-addon="">"@"</span>
                    <InputIsland placeholder="username" name="username" />
                </InputGroupIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Grouped inputs maintain consistent structure and visual merging."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Detached"</span>
                <div data-rs-showcase-preview-row="">
                    <InputGroupIsland>
                        <span data-rs-input-group-addon="">"@"</span>
                        <InputIsland placeholder="username" name="username-detached" />
                    </InputGroupIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"URL input"</span>
                <div data-rs-showcase-preview-row="">
                    <InputGroupIsland merge_radius=true>
                        <span data-rs-input-group-addon="">"https://"</span>
                        <InputIsland placeholder="example.com" name="url" />
                    </InputGroupIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With suffix"</span>
                <div data-rs-showcase-preview-row="">
                    <InputGroupIsland merge_radius=true>
                        <InputIsland placeholder="0.00" name="amount" />
                        <span data-rs-input-group-addon="">"USD"</span>
                    </InputGroupIsland>
                </div>
            </div>
        </div>
    }
}

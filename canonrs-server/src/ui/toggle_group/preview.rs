use leptos::prelude::*;
use super::toggle_group_boundary::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn ToggleGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ToggleGroup>
                    <ToggleGroupItem value="left" on=true>"Left"</ToggleGroupItem>
                    <ToggleGroupItem value="center">"Center"</ToggleGroupItem>
                    <ToggleGroupItem value="right">"Right"</ToggleGroupItem>
                </ToggleGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Group behavior and selection mode enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Multiple selection"</span>
                <div data-rs-showcase-preview-row="">
                    <ToggleGroup multiple=true>
                        <ToggleGroupItem value="bold" on=true>"Bold"</ToggleGroupItem>
                        <ToggleGroupItem value="italic" on=true>"Italic"</ToggleGroupItem>
                        <ToggleGroupItem value="underline">"Underline"</ToggleGroupItem>
                        <ToggleGroupItem value="strike">"Strike"</ToggleGroupItem>
                    </ToggleGroup>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <ToggleGroup disabled=true>
                        <ToggleGroupItem value="a">"Option A"</ToggleGroupItem>
                        <ToggleGroupItem value="b">"Option B"</ToggleGroupItem>
                    </ToggleGroup>
                </div>
            </div>
        </div>
    }
}

use leptos::prelude::*;
use super::toggle_group_ui::ToggleGroup;
use crate::ui::toggle::Toggle;

#[component]
pub fn ToggleGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ToggleGroup>
                    <Toggle pressed=true><span>{"Left"}</span></Toggle>
                    <Toggle><span>{"Center"}</span></Toggle>
                    <Toggle><span>{"Right"}</span></Toggle>
                </ToggleGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Group behavior and selection mode enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Multiple selection"</span>
                <div data-rs-showcase-preview-row="">
                    <ToggleGroup multiple=true>
                        <Toggle pressed=true><span>{"Bold"}</span></Toggle>
                        <Toggle pressed=true><span>{"Italic"}</span></Toggle>
                        <Toggle><span>{"Underline"}</span></Toggle>
                        <Toggle><span>{"Strike"}</span></Toggle>
                    </ToggleGroup>
                </div>
            </div>
        </div>
    }
}

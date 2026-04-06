use leptos::prelude::*;
use super::toggle_group_island::{ToggleGroupIsland, ToggleGroupItem};

#[component]
pub fn ToggleGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ToggleGroupIsland
                    items=vec![
                        ToggleGroupItem { value: "left".into(),   label: "Left".into(),   on: true,  disabled: false },
                        ToggleGroupItem { value: "center".into(), label: "Center".into(), on: false, disabled: false },
                        ToggleGroupItem { value: "right".into(),  label: "Right".into(),  on: false, disabled: false },
                    ]
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Group behavior and selection mode enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Multiple selection"</span>
                <div data-rs-showcase-preview-row="">
                    <ToggleGroupIsland
                        multiple=true
                        items=vec![
                            ToggleGroupItem { value: "bold".into(),      label: "Bold".into(),      on: true,  disabled: false },
                            ToggleGroupItem { value: "italic".into(),    label: "Italic".into(),    on: true,  disabled: false },
                            ToggleGroupItem { value: "underline".into(), label: "Underline".into(), on: false, disabled: false },
                            ToggleGroupItem { value: "strike".into(),    label: "Strike".into(),    on: false, disabled: false },
                        ]
                    />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <ToggleGroupIsland
                        disabled=true
                        items=vec![
                            ToggleGroupItem { value: "a".into(), label: "Option A".into(), on: false, disabled: false },
                            ToggleGroupItem { value: "b".into(), label: "Option B".into(), on: false, disabled: false },
                        ]
                    />
                </div>
            </div>
        </div>
    }
}

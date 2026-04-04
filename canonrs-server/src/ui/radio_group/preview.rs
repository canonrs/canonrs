use leptos::prelude::*;
use super::radio_group_island::{RadioGroupIsland, RadioOption};

#[component]
pub fn RadioGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <RadioGroupIsland
                    name="framework"
                    options=vec![
                        RadioOption { value: "leptos".into(), label: "Leptos".into(), disabled: false },
                        RadioOption { value: "dioxus".into(), label: "Dioxus".into(), disabled: false },
                        RadioOption { value: "yew".into(),    label: "Yew".into(),    disabled: false },
                    ]
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection state mapped directly to DOM and ARIA."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Pre-selected"</span>
                <div data-rs-showcase-preview-row="">
                    <RadioGroupIsland
                        name="size"
                        selected_value="md"
                        options=vec![
                            RadioOption { value: "sm".into(), label: "Small".into(),  disabled: false },
                            RadioOption { value: "md".into(), label: "Medium".into(), disabled: false },
                            RadioOption { value: "lg".into(), label: "Large".into(),  disabled: false },
                        ]
                    />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <RadioGroupIsland
                        name="dis"
                        disabled=true
                        options=vec![
                            RadioOption { value: "a".into(), label: "Option A".into(), disabled: false },
                            RadioOption { value: "b".into(), label: "Option B".into(), disabled: false },
                        ]
                    />
                </div>
            </div>

        </div>
    }
}

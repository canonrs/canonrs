use leptos::prelude::*;
use super::radio_island::{RadioGroupIsland, RadioOption};

#[component]
pub fn RadioShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <RadioGroupIsland
                    name="framework"
                    selected_value="dioxus"
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
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-preview-dev-grid="">
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Unselected"</span>
                        <RadioGroupIsland
                            name="state-a"
                            options=vec![
                                RadioOption { value: "a".into(), label: "Option A".into(), disabled: false },
                            ]
                        />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Selected"</span>
                        <RadioGroupIsland
                            name="state-b"
                            selected_value="b"
                            options=vec![
                                RadioOption { value: "b".into(), label: "Option B".into(), disabled: false },
                            ]
                        />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Disabled"</span>
                        <RadioGroupIsland
                            name="state-c"
                            options=vec![
                                RadioOption { value: "c".into(), label: "Option C".into(), disabled: true },
                            ]
                        />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Selected + Disabled"</span>
                        <RadioGroupIsland
                            name="state-d"
                            selected_value="d"
                            options=vec![
                                RadioOption { value: "d".into(), label: "Option D".into(), disabled: true },
                            ]
                        />
                    </div>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Group — pre-selected"</span>
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
                <span data-rs-showcase-preview-label="">"Group — disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <RadioGroupIsland
                        name="disabled"
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

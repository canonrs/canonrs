use leptos::prelude::*;
use super::input_group_island::{InputGroupIsland, InputGroupSlot};

#[component]
pub fn InputGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <InputGroupIsland
                    merge_radius=true
                    slots=vec![
                        InputGroupSlot::Addon("@".to_string()),
                        InputGroupSlot::Input {
                            placeholder: "username".to_string(),
                            input_type:  "text".to_string(),
                            name:        "username".to_string(),
                        },
                    ]
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Grouped inputs maintain consistent structure and visual merging."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Detached"</span>
                <div data-rs-showcase-preview-row="">
                    <InputGroupIsland
                        slots=vec![
                            InputGroupSlot::Addon("@".to_string()),
                            InputGroupSlot::Input {
                                placeholder: "username".to_string(),
                                input_type:  "text".to_string(),
                                name:        "username-detached".to_string(),
                            },
                        ]
                    />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"URL input"</span>
                <div data-rs-showcase-preview-row="">
                    <InputGroupIsland
                        merge_radius=true
                        slots=vec![
                            InputGroupSlot::Addon("https://".to_string()),
                            InputGroupSlot::Input {
                                placeholder: "example.com".to_string(),
                                input_type:  "text".to_string(),
                                name:        "url".to_string(),
                            },
                        ]
                    />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With suffix"</span>
                <div data-rs-showcase-preview-row="">
                    <InputGroupIsland
                        merge_radius=true
                        slots=vec![
                            InputGroupSlot::Input {
                                placeholder: "0.00".to_string(),
                                input_type:  "text".to_string(),
                                name:        "amount".to_string(),
                            },
                            InputGroupSlot::Addon("USD".to_string()),
                        ]
                    />
                </div>
            </div>

        </div>
    }
}

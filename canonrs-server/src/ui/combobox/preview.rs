use leptos::prelude::*;
use super::combobox_island::{ComboboxIsland, ComboboxOption};

#[component]
pub fn ComboboxShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ComboboxIsland
                    placeholder="Search framework..."
                    options=vec![
                        ComboboxOption { value: "leptos".into(),  label: "Leptos".into(),           disabled: false },
                        ComboboxOption { value: "dioxus".into(),  label: "Dioxus".into(),           disabled: false },
                        ComboboxOption { value: "yew".into(),     label: "Yew".into(),              disabled: false },
                        ComboboxOption { value: "react".into(),   label: "React (disabled)".into(), disabled: true  },
                    ]
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Combobox roles and interaction fully enforced by structure."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Pre-selected"</span>
                <div data-rs-showcase-preview-row="">
                    <ComboboxIsland
                        placeholder="Search size..."
                        selected_value="md"
                        options=vec![
                            ComboboxOption { value: "xs".into(), label: "Extra Small".into(), disabled: false },
                            ComboboxOption { value: "sm".into(), label: "Small".into(),       disabled: false },
                            ComboboxOption { value: "md".into(), label: "Medium".into(),      disabled: false },
                            ComboboxOption { value: "lg".into(), label: "Large".into(),       disabled: false },
                            ComboboxOption { value: "xl".into(), label: "Extra Large".into(), disabled: false },
                        ]
                    />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <ComboboxIsland
                        placeholder="Disabled..."
                        disabled=true
                        options=vec![
                            ComboboxOption { value: "a".into(), label: "Option A".into(), disabled: false },
                        ]
                    />
                </div>
            </div>

        </div>
    }
}

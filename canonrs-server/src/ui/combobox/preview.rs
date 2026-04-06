use leptos::prelude::*;
use super::combobox_island::{ComboboxIsland, ComboboxOption};
use super::combobox_ui::{Combobox, ComboboxInput, ComboboxList, ComboboxItem};
use canonrs_core::meta::DisabledState;

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
                    // SSR: item selecionado via selected=Selected, input reflete label
                    <Combobox>
                        <ComboboxInput placeholder="Search size..." />
                        <ComboboxList>
                            <ComboboxItem value="xs">"Extra Small"</ComboboxItem>
                            <ComboboxItem value="sm">"Small"</ComboboxItem>
                            <ComboboxItem value="md" selected=canonrs_core::meta::SelectionState::Selected>"Medium"</ComboboxItem>
                            <ComboboxItem value="lg">"Large"</ComboboxItem>
                            <ComboboxItem value="xl">"Extra Large"</ComboboxItem>
                        </ComboboxList>
                    </Combobox>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <Combobox disabled=DisabledState::Disabled>
                        <ComboboxInput placeholder="Disabled..." disabled=DisabledState::Disabled />
                        <ComboboxList>
                            <ComboboxItem value="a">"Option A"</ComboboxItem>
                        </ComboboxList>
                    </Combobox>
                </div>
            </div>

        </div>
    }
}

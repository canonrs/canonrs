use leptos::prelude::*;
use super::combobox_boundary::Combobox;
use super::combobox_ui::{ComboboxInput, ComboboxList, ComboboxItem};
use canonrs_core::meta::{DisabledState, SelectionState};

#[component]
pub fn ComboboxShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Combobox placeholder="Search framework...">
                    <ComboboxItem value="leptos">"Leptos"</ComboboxItem>
                    <ComboboxItem value="dioxus">"Dioxus"</ComboboxItem>
                    <ComboboxItem value="yew">"Yew"</ComboboxItem>
                    <ComboboxItem value="react" disabled=DisabledState::Disabled>"React (disabled)"</ComboboxItem>
                </Combobox>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Combobox roles and interaction fully enforced by structure."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Pre-selected"</span>
                <div data-rs-showcase-preview-row="">
                    <Combobox>
                        <ComboboxInput placeholder="Search size..." />
                        <ComboboxList>
                            <ComboboxItem value="xs">"Extra Small"</ComboboxItem>
                            <ComboboxItem value="sm">"Small"</ComboboxItem>
                            <ComboboxItem value="md" selected=SelectionState::Selected>"Medium"</ComboboxItem>
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

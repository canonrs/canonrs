use leptos::prelude::*;
use super::combobox_boundary::{Combobox, ComboboxInput, ComboboxList, ComboboxItem};
use canonrs_core::meta::{DisabledState, SelectionState};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ComboboxShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Combobox placeholder="Search framework...">
                <ComboboxItem value="leptos">"Leptos"</ComboboxItem>
                <ComboboxItem value="dioxus">"Dioxus"</ComboboxItem>
                <ComboboxItem value="yew">"Yew"</ComboboxItem>
                <ComboboxItem value="react" disabled=DisabledState::Disabled>"React (disabled)"</ComboboxItem>
            </Combobox>
            <p data-rs-showcase-preview-anchor="">
                "Combobox roles and interaction fully enforced by structure."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Pre-selected"</span>
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
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <Combobox disabled=DisabledState::Disabled>
                    <ComboboxInput placeholder="Disabled..." disabled=DisabledState::Disabled />
                    <ComboboxList>
                        <ComboboxItem value="a">"Option A"</ComboboxItem>
                    </ComboboxList>
                </Combobox>
            </Stack>
        </Stack>
    }
}

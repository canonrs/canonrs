//! Combobox Island — Canon Rule passthrough
use leptos::prelude::*;
pub use super::combobox_ui::{ComboboxInput, ComboboxList, ComboboxItem};

#[component]
pub fn Combobox(
    children: Children,
    #[prop(into, default = String::from("Search..."))] placeholder: String,
    #[prop(default = canonrs_core::meta::DisabledState::Enabled)] disabled: canonrs_core::meta::DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <super::combobox_ui::Combobox disabled=disabled class=class>
            <ComboboxInput placeholder=placeholder />
            <ComboboxList>
                {children()}
            </ComboboxList>
        </super::combobox_ui::Combobox>
    }
}

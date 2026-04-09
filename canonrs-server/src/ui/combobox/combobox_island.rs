//! @canon-level: strict
//! Combobox Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::combobox_ui::{Combobox, ComboboxInput, ComboboxList, ComboboxItem};
use canonrs_core::meta::DisabledState;

#[derive(Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}



#[component]
pub fn ComboboxIsland(
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    options: Vec<ComboboxOption>,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_else(|| "Search...".to_string());
    let class       = class.unwrap_or_default();

    view! {
        <Combobox class=class>
            <ComboboxInput placeholder=placeholder />
            <ComboboxList>
                {options.into_iter().map(|opt| {
                    let disabled = if opt.disabled { DisabledState::Disabled } else { DisabledState::Enabled };
                    view! {
                        <ComboboxItem value=opt.value disabled=disabled>{opt.label}</ComboboxItem>
                    }
                }).collect::<Vec<_>>()}
            </ComboboxList>
        </Combobox>
    }
}

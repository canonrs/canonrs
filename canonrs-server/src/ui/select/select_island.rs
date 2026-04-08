//! @canon-level: strict
//! Select Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::select_ui::{Select, SelectTrigger, SelectValue, SelectContent, SelectItem};
use canonrs_core::meta::DisabledState;

#[derive(Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[island]
pub fn SelectInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
        });
    }
    view! { <></> }
}

#[component]
pub fn SelectIsland(
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    options: Vec<SelectOption>,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_else(|| "Select...".to_string());
    let class       = class.unwrap_or_default();

    let options_view = options.into_iter().map(|opt| {
        let disabled = if opt.disabled { DisabledState::Disabled } else { DisabledState::Enabled };
        view! { <SelectItem value=opt.value disabled=disabled>{opt.label}</SelectItem> }
    }).collect::<Vec<_>>();

    view! {
        <SelectInit />
        <Select class=class>
            <SelectTrigger>
                <SelectValue placeholder=placeholder>{""}</SelectValue>
            </SelectTrigger>
            <SelectContent>{options_view}</SelectContent>
        </Select>
    }
}

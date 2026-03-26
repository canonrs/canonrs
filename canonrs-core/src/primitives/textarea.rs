//! @canon-level: strict
//! @canon-owner: primitives-team
//! Textarea Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;
use crate::state_engine::disabled_attrs;

#[component]
pub fn TextareaPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Option<String>,
    #[prop(optional)] rows: Option<u32>,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    view! {
        <textarea
            data-rs-textarea=""
            data-rs-component="Textarea"
            data-rs-behavior="form"
            data-rs-disabled=d.data_rs_disabled
            data-rs-readonly={if readonly { Some("") } else { None }}
            data-rs-required={if required { Some("") } else { None }}
            prop:value=value
            placeholder={if placeholder.is_empty() { None } else { Some(placeholder) }}
            name={if name.is_empty() { None } else { Some(name) }}
            disabled=d.disabled
            aria-disabled=d.aria_disabled
            readonly=readonly
            aria-readonly={if readonly { Some("true") } else { None }}
            aria-required={if required { Some("true") } else { None }}
            aria-labelledby=aria_labelledby
            aria-describedby=aria_describedby
            rows=rows
            class=class
        />
    }
}

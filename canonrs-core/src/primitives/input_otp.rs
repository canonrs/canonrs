//! @canon-level: strict
//! @canon-owner: primitives-team
//! InputOtp Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;
use crate::infra::state_engine::disabled_attrs;

#[component]
pub fn InputOtpPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = 1u32)] maxlength: u32,
    #[prop(into, default = String::new())] pattern: String,
    #[prop(into, default = String::new())] inputmode: String,
    #[prop(into, default = String::new())] autocomplete: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    view! {
        <input
            data-rs-input-otp=""
            data-rs-uid=crate::infra::uid::generate("otp")
            data-rs-interaction="init"
            data-rs-component="InputOtp"
            data-rs-disabled=d.data_rs_disabled
            type="text"
            name={if name.is_empty() { None } else { Some(name) }}
            prop:value=value
            disabled=d.disabled
            maxlength=maxlength.to_string()
            pattern={if pattern.is_empty() { None } else { Some(pattern) }}
            inputmode={if inputmode.is_empty() { None } else { Some(inputmode) }}
            autocomplete={if autocomplete.is_empty() { None } else { Some(autocomplete) }}
            aria-disabled=d.aria_disabled
            class=class
        />
    }
}

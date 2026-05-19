//! @canon-level: strict
//! @canon-owner: primitives-team
//! InputOtp Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;

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
    let uid_otp = crate::infra::uid::generate("otp");
    view! {
        <input
            data-rs-input-otp=""
            data-rs-uid=uid_otp
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            type="text"
            name={if name.is_empty() { None } else { Some(name) }}
            value=value
            disabled=disabled.disabled()
            maxlength=maxlength.to_string()
            pattern={if pattern.is_empty() { None } else { Some(pattern) }}
            inputmode={if inputmode.is_empty() { None } else { Some(inputmode) }}
            autocomplete={if autocomplete.is_empty() { None } else { Some(autocomplete) }}
            aria-disabled=disabled.aria_disabled()
            class=class
        />
    }
}

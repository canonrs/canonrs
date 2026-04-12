//! @canon-level: strict
//! InputOtp Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::input_otp_ui::InputOtp as InputOtpUi;
use canonrs_core::meta::DisabledState;

#[component]
pub fn InputOtp(
    #[prop(default = 6)] length: u32,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] initial_value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let disabled_state = if disabled { DisabledState::Disabled
} else { DisabledState::Enabled };
    let val = if !initial_value.is_empty() { initial_value } else { value };
    view! {
        <InputOtpUi length=length name=name value=val disabled=disabled_state class=class />
    }
}

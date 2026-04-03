use leptos::prelude::*;
use canonrs_core::primitives::{InputOtpPrimitive, InputOtpSlotPrimitive};
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn InputOtp(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = 6u32)] length: u32,
) -> impl IntoView {
    let is_disabled = disabled == DisabledState::Disabled;
    let slots: Vec<AnyView> = (0..length).map(|i| {
        let ch = value.chars().nth(i as usize).map(|c| c.to_string()).unwrap_or_default();
        let state = if !is_disabled && i == value.len() as u32 {
            ActivityState::Active
        } else {
            ActivityState::Inactive
        };
        view! {
            <InputOtpSlotPrimitive state=state>
                {ch}
            </InputOtpSlotPrimitive>
        }.into_any()
    }).collect();

    let disabled_state: Option<&str> = if is_disabled { Some("disabled") } else { None };

    view! {
        <div data-rs-input-otp-container="" data-rs-state=disabled_state class=class>
            <InputOtpPrimitive
                name=name
                value=value.clone()
                disabled=disabled
                maxlength=length
                inputmode="numeric".to_string()
                autocomplete="one-time-code".to_string()
            />
            <div data-rs-input-otp-slots="">
                {slots}
            </div>
        </div>
    }
}

#[component]
pub fn InputOtpPreview() -> impl IntoView {
    view! { <InputOtp length=6 /> }
}


use leptos::prelude::*;
use canonrs_core::primitives::InputOtpSlotPrimitive;
use canonrs_core::meta::{ActivityState, DisabledState};

fn make_slot(ch: String, state: ActivityState) -> impl IntoView {
    view! {
        <InputOtpSlotPrimitive state=state class="input-otp-slot".to_string()>
            {ch}
        </InputOtpSlotPrimitive>
    }
}

#[component]
pub fn InputOtp(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = 6)] length: u32,
) -> impl IntoView {
    let container_class = format!("input-otp-container {}", class);
    let is_disabled = disabled == DisabledState::Disabled;
    let slots: Vec<_> = (0..length).map(|i| {
        let ch = value.chars().nth(i as usize).map(|c| c.to_string()).unwrap_or_default();
        let state = if i == value.len() as u32 { ActivityState::Active } else { ActivityState::Inactive };
        make_slot(ch, state)
    }).collect();
    view! {
        <div class={container_class}>
            <input
                data-rs-input-otp=""
                type="text"
                name=name
                prop:value=value
                disabled=is_disabled
                maxlength=length.to_string()
                inputmode="numeric"
                autocomplete="one-time-code"
                class="input-otp-hidden"
            />
            <div class="input-otp-slots">
                {slots}
            </div>
        </div>
    }
}

#[component]
pub fn InputOtpPreview() -> impl IntoView {
    view! { <InputOtp length=6 /> }
}

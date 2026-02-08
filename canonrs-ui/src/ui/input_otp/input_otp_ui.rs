use leptos::prelude::*;
use super::input_otp_primitive::InputOtpPrimitive;
use super::input_otp_slot_primitive::InputOtpSlotPrimitive;

#[component]
pub fn InputOtp(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = 6)] length: u32,
) -> impl IntoView {
    let container_class = format!("input-otp-container {}", class);

    let slots = (0..length).map(|i| {
        let char = value.chars().nth(i as usize).map(|c| c.to_string()).unwrap_or_default();
        let is_active = i == value.len() as u32;

        view! {
            <InputOtpSlotPrimitive
                class="input-otp-slot".to_string()
                char={char}
                is_active={is_active}
            />
        }
    }).collect_view();

    view! {
        <div class={container_class}>
            <InputOtpPrimitive
                id={id}
                class="input-otp-hidden".to_string()
                name={name}
                value={value.clone()}
                disabled={disabled}
                maxlength={length}
                pattern="[0-9]*".to_string()
                inputmode="numeric".to_string()
                autocomplete="one-time-code".to_string()
            />
            <div class="input-otp-slots">
                {slots}
            </div>
        </div>
    }
}

use leptos::prelude::*;

#[component]
pub fn InputOtpSlotPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] char: String,
    #[prop(default = false)] is_active: bool,
) -> impl IntoView {
    view! {
        <div
            class={class}
            attr:data-input-otp-slot=""
            attr:data-active={if is_active { "true" } else { "" }}
        >
            {char}
        </div>
    }
}

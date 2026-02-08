use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::ev;
use canonrs_ui::ui::input_otp::InputOtp;

#[component]
pub fn InputOtpInteractive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = 6)] length: u32,
    #[prop(optional)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    view! {
        <InputOtp
            id=id
            class=class
            name=name
            value=value
            disabled=disabled
            length=length
            on:input=move |ev: ev::Event| {
                if let Some(ref handler) = on_change {
                    handler.run(event_target_value(&ev));
                }
            }
        />
    }
}

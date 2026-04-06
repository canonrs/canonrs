use leptos::prelude::*;
use super::input_otp_ui::InputOtp;
use canonrs_core::meta::DisabledState;

#[island]
pub fn InputOtpIsland(
    #[prop(optional)] length: Option<u32>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] initial_value: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let length   = length.unwrap_or(6);
    let name     = name.unwrap_or_default();
    let class    = class.unwrap_or_default();
    let is_disabled = disabled.unwrap_or(false);
    let init     = initial_value.unwrap_or_default();

    let disabled_state = if is_disabled { DisabledState::Disabled } else { DisabledState::Enabled };



    let container_ref = NodeRef::<leptos::html::Div>::new();

    let on_click = move |_: leptos::ev::MouseEvent| {
        #[cfg(feature = "hydrate")]
        {
            use leptos::wasm_bindgen::JsCast;
            if let Some(container) = container_ref.get() {
                if let Ok(Some(input)) = container.query_selector("[data-rs-input-otp]") {
                    if let Ok(el) = input.dyn_into::<leptos::web_sys::HtmlInputElement>() {
                        let _ = el.focus();
                    }
                }
            }
        }
    };



    view! {
        <div node_ref=container_ref on:click=on_click>
            <InputOtp
                length=length
                name=name
                value=init
                disabled=disabled_state
                class=class
            />
        </div>
    }
}

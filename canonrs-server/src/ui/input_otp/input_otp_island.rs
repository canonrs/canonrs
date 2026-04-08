//! InputOtp Island — Canon Rule #341
//! DOM-driven, zero state. Lógica via web_sys + Effect.

use leptos::prelude::*;
use super::input_otp_ui::InputOtp;
use canonrs_core::meta::DisabledState;

#[island]
pub fn InputOtpIsland(
    #[prop(optional)] length:               Option<u32>,
    #[prop(optional, into)] name:           Option<String>,
    #[prop(optional, into)] initial_value:  Option<String>,
    #[prop(optional, into)] class:          Option<String>,
    #[prop(optional)] disabled:             Option<bool>,
) -> impl IntoView {
    let length      = length.unwrap_or(6);
    let name        = name.unwrap_or_default();
    let class       = class.unwrap_or_default();
    let is_disabled = disabled.unwrap_or(false);
    let init        = initial_value.unwrap_or_default();
    let disabled_state = if is_disabled { DisabledState::Disabled } else { DisabledState::Enabled };

    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();

        if root.has_attribute("data-rs-attached") { return; }
        let _ = root.set_attribute("data-rs-attached", "1");

        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(_)>::wrap(Box::new(move |_: web_sys::MouseEvent| {
            if let Ok(Some(input)) = root_cb.query_selector("[data-rs-input-otp]") {
                if let Ok(el) = input.dyn_into::<web_sys::HtmlInputElement>() {
                    let _ = el.focus();
                }
            }
        }));
        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    view! {
        <div node_ref=node_ref>
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

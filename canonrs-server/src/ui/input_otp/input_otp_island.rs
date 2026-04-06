use leptos::prelude::*;
use leptos::web_sys;
use canonrs_core::primitives::{InputOtpPrimitive, InputOtpSlotPrimitive};
use canonrs_core::meta::{ActivityState, DisabledState};

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
    let disabled = disabled.unwrap_or(false);

    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };

    let init = initial_value.unwrap_or_default();
    let value = RwSignal::new(init.clone());

    let container_ref = NodeRef::<leptos::html::Div>::new();

    let on_click = move |_: leptos::ev::MouseEvent| {
        if let Some(container) = container_ref.get() {
            use leptos::wasm_bindgen::JsCast;
            if let Ok(Some(input)) = container.query_selector("[data-rs-input-otp]") {
                if let Ok(el) = input.dyn_into::<web_sys::HtmlInputElement>() {
                    let _ = el.focus();
                }
            }
        }
    };

    let on_input = move |e: leptos::ev::Event| {
        use leptos::wasm_bindgen::JsCast;
        if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
            value.set(input.value());
        }
    };

    let slots_view = move || {
        let v = value.get();
        (0..length).map(|i| {
            let ch = v.chars().nth(i as usize).map(|c| c.to_string()).unwrap_or_default();
            let state = if !disabled && i == v.len() as u32 {
                ActivityState::Active
            } else {
                ActivityState::Inactive
            };
            view! {
                <InputOtpSlotPrimitive state=state>
                    {ch}
                </InputOtpSlotPrimitive>
            }
        }).collect::<Vec<_>>()
    };

    view! {
        <div
            node_ref=container_ref
            data-rs-input-otp-container=""
            data-rs-state=if disabled { Some("disabled") } else { None }
            class=class
            on:click=on_click
        >
            <InputOtpPrimitive
                name=name
                value=init
                disabled=disabled_state
                maxlength=length
                inputmode="numeric".to_string()
                autocomplete="one-time-code".to_string()
                on:input=on_input
            />
            <div data-rs-input-otp-slots="">
                {slots_view}
            </div>
        </div>
    }
}

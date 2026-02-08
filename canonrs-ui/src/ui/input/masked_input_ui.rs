use leptos::prelude::*;
use leptos::either::Either;
use crate::primitives::input::InputPrimitive;

#[component]
pub fn MaskedInput(
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] _mask: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    if let Some(handler) = on_input {
        Either::Left(view! {
            <InputPrimitive
                value={value}
                placeholder={placeholder}
                disabled={disabled}
                class={class}
                id={id}
                on:input=move |ev| handler.run(event_target_value(&ev))
            />
        })
    } else {
        Either::Right(view! {
            <InputPrimitive
                value={value}
                placeholder={placeholder}
                disabled={disabled}
                class={class}
                id={id}
            />
        })
    }
}

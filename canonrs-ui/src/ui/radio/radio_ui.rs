use leptos::prelude::*;
use crate::primitives::RadioPrimitive;

#[component]
pub fn Radio(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <RadioPrimitive
            checked={checked}
            disabled={disabled}
            value={value}
            name={name}
            class={class}
            id={id.unwrap_or_default()}
        >
            {children.map(|c| c())}
        </RadioPrimitive>
    }
}

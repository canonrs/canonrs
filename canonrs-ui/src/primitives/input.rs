//! @canon-level: strict
//! @canon-owner: primitives-team
//! Input Primitive - HTML puro
//! CANON RULE: value={value} = controlled input (sempre controlado)

use leptos::prelude::*;

#[component]
pub fn InputPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = "text".to_string())] input_type: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] aria_label: String,
) -> impl IntoView {
    view! {
        <input
            type={input_type}
            class={class}
            id={id}
            name={name}
            value={value}
            placeholder={if placeholder.is_empty() { None } else { Some(placeholder) }}
            attr:aria-disabled={if disabled { "true" } else { "false" }}
            attr:data-disabled={if disabled { Some("true") } else { None }}
            attr:aria-label={if aria_label.is_empty() { None } else { Some(aria_label) }}
        />
    }
}

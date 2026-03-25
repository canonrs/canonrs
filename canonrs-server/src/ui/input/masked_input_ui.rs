//! @canon-level: ui
//! MaskedInput - sem behavior

use leptos::prelude::*;
use canonrs_core::primitives::InputPrimitive;

#[component]
pub fn MaskedInput(
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] mask: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <InputPrimitive
            value=value
            placeholder=placeholder
            disabled=disabled.into()
            class=class
            attr:data-rs-mask=mask
        />
    }
}

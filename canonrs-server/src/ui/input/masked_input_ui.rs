//! @canon-id: masked-input
//! @canon-label: Masked Input
//! @canon-family: input
//! @canon-category: Form
//! @canon-intent: Capture text with a format mask applied
//! @canon-description: Text input with format mask
//! @canon-composable: false
//! @canon-capabilities: Value, Disabled
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: masked-input, mask, format, phone, cpf, field

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

//! @canon-level: ui
//! Radio - native HTML input, sem behavior
//! Estado gerenciado pelo browser via :checked/:disabled

use leptos::prelude::*;
use canonrs_core::primitives::RadioPrimitive;

#[component]
pub fn Radio(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
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

#[component]
pub fn RadioPreview() -> impl IntoView {
    view! {
        <Radio value="opt1" name="radio-preview">"Option 1"</Radio>
        <Radio value="opt2" name="radio-preview">"Option 2"</Radio>
    }
}

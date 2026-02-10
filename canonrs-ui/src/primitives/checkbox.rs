//! @canon-level: strict
//! Checkbox Primitive - Native HTML input + CSS

use leptos::prelude::*;

#[component]
pub fn CheckboxPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            data-checkbox-input=""
            checked=checked
            disabled=disabled
            name=name
            value=value
            class=class
            id=id
        />
    }
}

#[component]
pub fn CheckboxIndicatorPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-checkbox-indicator="" class=class>
            {children.map(|c| c())}
        </span>
    }
}

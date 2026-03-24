//! @canon-level: strict
//! @canon-owner: primitives-team
//! Checkbox Primitive - Native HTML input + CSS

use leptos::prelude::*;

#[component]
pub fn CheckboxPrimitive(
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            data-rs-checkbox-input=""
            checked=checked
            disabled=disabled
            name=name
            class=class
            id=id
        />
    }
}

#[component]
pub fn CheckboxIndicatorPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-checkbox-indicator="" class=class>
            {children.map(|c| c())}
        </span>
    }
}

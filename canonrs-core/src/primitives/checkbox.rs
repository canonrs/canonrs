//! @canon-level: strict
//! Checkbox Primitive - Native HTML input + CSS

use leptos::prelude::*;

#[component]
pub fn CheckboxPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(into, default = Signal::derive(|| false))] disabled: Signal<bool>,
    #[prop(default = String::new())] name: String,
    #[prop(into, default = Signal::derive(|| String::new()))] value: Signal<String>,
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
            prop:value=move || value.get()
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

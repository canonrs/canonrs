//! @canon-level: strict
//! Switch Primitive - native checkbox + CSS

use leptos::prelude::*;

#[component]
pub fn SwitchPrimitive(
    children: Children,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <label data-rs-switch="" class=class>
            <input
                type="checkbox"
                checked=checked
                disabled=disabled
                name={if name.is_empty() { None } else { Some(name) }}
                prop:value=value
            />
            {children()}
        </label>
    }
}

#[component]
pub fn SwitchThumbPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <span data-rs-switch-thumb="" class=class /> }
}

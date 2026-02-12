use leptos::prelude::*;
use crate::primitives::{SwitchPrimitive, SwitchThumbPrimitive};

#[component]
pub fn Switch(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <SwitchPrimitive
            checked={checked}
            disabled={disabled}
            name={name}
            value={value}
            class={class}
            id={id.unwrap_or_default()}
        >
            <SwitchThumb />
            {children.map(|c| c())}
        </SwitchPrimitive>
    }
}

#[component]
pub fn SwitchThumb() -> impl IntoView {
    view! {
        <SwitchThumbPrimitive />
    }
}

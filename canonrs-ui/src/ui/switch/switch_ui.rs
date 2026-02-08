use leptos::prelude::*;
use crate::primitives::{
    SwitchPrimitive,
    SwitchThumbPrimitive,
};

#[component]
pub fn Switch(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SwitchPrimitive
            attr:data-checked={if checked { "true" } else { "" }}
            checked=checked
            disabled=disabled
            name=name
            value=value
            class=class
            id=id
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

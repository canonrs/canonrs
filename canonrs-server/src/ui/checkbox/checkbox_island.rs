use leptos::prelude::*;
use super::checkbox_ui::CheckboxReactive;

#[island]
pub fn CheckboxIsland(
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let initial  = checked.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let name     = name.unwrap_or_default();
    let class    = class.unwrap_or_default();
    let checked  = RwSignal::new(initial);

    view! {
        <CheckboxReactive
            checked=checked
            disabled=disabled
            name=name
            class=class
        >
            {children()}
        </CheckboxReactive>
    }
}

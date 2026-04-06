use leptos::prelude::*;
use super::switch_ui::SwitchReactive;

#[island]
pub fn SwitchIsland(
    #[prop(optional)] checked: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let initial  = checked.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let name     = name.unwrap_or_default();
    let value    = value.unwrap_or_default();
    let class    = class.unwrap_or_default();

    let checked = RwSignal::new(initial);

    view! {
        <SwitchReactive
            checked=checked
            disabled=disabled
            name=name
            value=value
            class=class
        />
    }
}

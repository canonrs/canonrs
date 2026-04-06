use leptos::prelude::*;
use super::toggle_ui::ToggleReactive;

#[island]
pub fn ToggleIsland(
    #[prop(optional)] pressed: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let initial    = pressed.unwrap_or(false);
    let disabled   = disabled.unwrap_or(false);
    let aria_label = aria_label.unwrap_or_default();
    let class      = class.unwrap_or_default();
    let pressed    = RwSignal::new(initial);

    view! {
        <ToggleReactive
            pressed=pressed
            disabled=disabled
            aria_label=aria_label
            class=class
        >
            {children()}
        </ToggleReactive>
    }
}

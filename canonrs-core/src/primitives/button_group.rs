//! @canon-level: strict
//! @canon-owner: primitives-team
//! ButtonGroup Primitive - HTML puro

use leptos::prelude::*;
use crate::ToggleState;

#[component]
pub fn ButtonGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = ToggleState::Off)] attached: ToggleState,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-button-group=""
            data-rs-component="ButtonGroup"
            data-rs-behavior="action"
            data-rs-attached={if attached == ToggleState::On { Some("true") } else { None }}
            role="group"
            aria-label=aria_label
            class=class
        >
            {children()}
        </div>
    }
}

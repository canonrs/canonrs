//! @canon-level: strict
//! @canon-owner: primitives-team
//! ToggleGroup Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;

#[component]
pub fn ToggleGroupPrimitive(
    children: Children,
    #[prop(default = false)] multiple: bool,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let state_str = if disabled == DisabledState::Disabled { "disabled" } else { "" };
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    view! {
        <div
            data-rs-toggle-group=""
            data-rs-component="ToggleGroup"
            data-rs-multiple=if multiple { "true" } else { "false" }
            data-rs-state=state_str
            role="group"
            aria-disabled=aria_disabled
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

//! @canon-level: strict
//! @canon-owner: primitives-team
//! ToggleGroup Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::DisabledState;
use crate::state_engine::disabled_attrs;

#[component]
pub fn ToggleGroupPrimitive(
    children: Children,
    #[prop(default = false)] multiple: bool,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d = disabled_attrs(disabled);
    view! {
        <div
            data-rs-toggle-group=""
            data-rs-component="ToggleGroup"
            data-rs-behavior="toggle"
            data-rs-multiple=if multiple { "true" } else { "false" }
            data-rs-disabled=d.data_rs_disabled
            role="group"
            aria-disabled=d.aria_disabled
            class=class
        >
            {children()}
        </div>
    }
}

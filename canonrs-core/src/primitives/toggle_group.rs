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
    let uid_tg = crate::infra::uid::generate("tg");
    let is_disabled = disabled == DisabledState::Disabled;

    view! {
        <div
            data-rs-toggle-group=""
            data-rs-uid=uid_tg
            data-rs-interaction="selection"
            data-rs-multiple=if multiple { "true" } else { "false" }
            data-rs-disabled=if is_disabled { Some("disabled") } else { None }
            role="group"
            aria-disabled=if is_disabled { "true" } else { "false" }
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

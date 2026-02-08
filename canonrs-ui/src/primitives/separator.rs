//! @canon-level: strict
//! @canon-owner: primitives-team
//! Separator Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn SeparatorPrimitive(
    #[prop(default = "horizontal".to_string())] orientation: String,
    #[prop(default = true)] decorative: bool,
    #[prop(default = String::new())] aria_label: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-separator=""
            attr:data-orientation={orientation.clone()}
            role={if decorative { "presentation" } else { "separator" }}
            attr:aria-orientation={if !decorative { Some(orientation) } else { None }}
            attr:aria-label={if !decorative && !aria_label.is_empty() { Some(aria_label) } else { None }}
            class={class}
            id={id}
        />
    }
}

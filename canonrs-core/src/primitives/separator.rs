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
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let role = if decorative { "presentation" } else { "separator" };
    let aria_orientation = if !decorative { Some(orientation.clone()) } else { None };
    let aria_label_val = if !decorative && !aria_label.is_empty() { Some(aria_label) } else { None };

    view! {
        <div
            data-separator=""
            data-orientation={orientation}
            role={role}
            aria-orientation={aria_orientation}
            aria-label={aria_label_val}
            class={class}
            id={id}
        />
    }
}

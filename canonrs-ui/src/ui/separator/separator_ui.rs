use leptos::prelude::*;
use crate::primitives::SeparatorPrimitive;
use crate::shared::Orientation;

#[component]
pub fn Separator(
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = true)] decorative: bool,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-separator=""
            data-orientation={orientation.as_str()}
            role={if decorative { "presentation" } else { "separator" }}
            aria-orientation={if !decorative { Some(orientation.as_str()) } else { None }}
            aria-label={if !decorative && !aria_label.is_empty() { Some(aria_label) } else { None }}
            class={class}
            id={id}
        />
    }
}

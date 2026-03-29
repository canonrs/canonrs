//! @canon-id: separator
//! @canon-label: Separator
//! @canon-family: layout
//! @canon-category: Layout
//! @canon-intent: Visually divide content sections
//! @canon-description: Visual divider line
//! @canon-composable: false
//! @canon-capabilities: Orientation
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: separator, divider, line, hr, section

use leptos::prelude::*;
use canonrs_core::Orientation;

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
            data-rs-separator=""
            data-orientation={orientation.as_str()}
            role={if decorative { "presentation" } else { "separator" }}
            aria-orientation={if !decorative { Some(orientation.as_str()) } else { None }}
            aria-label={if !decorative && !aria_label.is_empty() { Some(aria_label) } else { None }}
            class={class}
            id={id}
        />
    }
}

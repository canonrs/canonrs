//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn SeparatorPrimitive(
    #[prop(default = "horizontal".to_string(), into)] orientation: String,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            role="separator"
            aria-orientation=orientation.clone()
            data-orientation=orientation
            class=class
        />
    }
}

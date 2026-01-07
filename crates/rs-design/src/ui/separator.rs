//! Separator Component
//!
//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Pre-token legacy component
//! @canon-owner: ui-team
//! @canon-target-date: 2025-03-01
//! @canon-migration-status: planned

use crate::primitives::separator::SeparatorPrimitive;
use leptos::prelude::*;

#[component]
pub fn Separator(
    #[prop(default = "horizontal".to_string(), into)] orientation: String,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let orientation_class = if orientation == "vertical" {
        "h-full w-px"
    } else {
        "h-[2px]"
    };

    let classes = format!(
        "shrink-0 bg-gray-300 {} {}",
        orientation_class,
        class
    );

    view! {
        <SeparatorPrimitive
            orientation=orientation
            class=classes
        />
    }
}

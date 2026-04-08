//! Separator Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::separator_ui::Separator;
use canonrs_core::Orientation;

#[component]
pub fn SeparatorIsland(
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(default = true)] decorative:                     bool,
    #[prop(into, default = String::new())] aria_label:      String,
    #[prop(into, default = String::new())] class:           String,
    #[prop(optional)] id:                                  Option<String>,
) -> impl IntoView {
    view! {
        <Separator
            orientation=orientation
            decorative=decorative
            aria_label=aria_label
            class=class
            id=id.unwrap_or_default()
        />
    }
}

//! Separator Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::separator_ui::Separator as SeparatorUi;
use canonrs_core::separator::SeparatorOrientation;

#[component]
pub fn Separator(
    #[prop(default = SeparatorOrientation::Horizontal)] orientation: SeparatorOrientation,
    #[prop(default = true)] decorative:                     bool,
    #[prop(into, default = String::new())] aria_label:      String,
    #[prop(into, default = String::new())] class:           String,
    #[prop(optional)] id:                                  Option<String>,
) -> impl IntoView {
    view! {
        <SeparatorUi
            orientation=orientation
            decorative=decorative
            aria_label=aria_label
            class=class
            id=id.unwrap_or_default()
        />
};
}

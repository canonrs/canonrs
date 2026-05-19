#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::SeparatorPrimitive;
use canonrs_core::separator::SeparatorOrientation;

#[component]
pub fn Separator(#[prop(default = SeparatorOrientation::Horizontal)] orientation: SeparatorOrientation, #[prop(default = true)] decorative: bool, #[prop(into, default = String::new())] aria_label: String, #[prop(default = String::new())] class: String, #[prop(into, optional)] id: Option<String>) -> impl IntoView {
    view! { <SeparatorPrimitive orientation=orientation decorative=decorative aria_label=aria_label class=class /> }
}

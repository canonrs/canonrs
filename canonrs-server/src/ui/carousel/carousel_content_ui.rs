#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::CarouselContentPrimitive;

#[component]
pub fn CarouselContent(
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! { <CarouselContentPrimitive class=class>{children.map(|c| c())}</CarouselContentPrimitive> }
}

#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::hero::{
    HeroMediaPrimitive, HeroActionsPrimitive,
    HeroTitlePrimitive, HeroSubtitlePrimitive, HeroDescriptionPrimitive,
};

#[component]
pub fn HeroTitle(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <HeroTitlePrimitive class=class>{children()}</HeroTitlePrimitive> }
}
#[component]
pub fn HeroSubtitle(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <HeroSubtitlePrimitive class=class>{children()}</HeroSubtitlePrimitive> }
}
#[component]
pub fn HeroDescription(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <HeroDescriptionPrimitive class=class>{children()}</HeroDescriptionPrimitive> }
}
#[component]
pub fn HeroMedia(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <HeroMediaPrimitive class=class>{children()}</HeroMediaPrimitive> }
}
#[component]
pub fn HeroActions(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <HeroActionsPrimitive class=class>{children()}</HeroActionsPrimitive> }
}

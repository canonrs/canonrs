#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::hero::{HeroMediaPrimitive, HeroActionsPrimitive};

#[component]
pub fn HeroTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h1 data-rs-hero-title="" class=class>{children()}</h1>
    }
}

#[component]
pub fn HeroSubtitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p data-rs-hero-subtitle="" class=class>{children()}</p>
    }
}

#[component]
pub fn HeroDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p data-rs-hero-description="" class=class>{children()}</p>
    }
}

#[component]
pub fn HeroMedia(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <HeroMediaPrimitive class=class>{children()}</HeroMediaPrimitive>
    }
}

#[component]
pub fn HeroActions(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <HeroActionsPrimitive class=class>{children()}</HeroActionsPrimitive>
    }
}

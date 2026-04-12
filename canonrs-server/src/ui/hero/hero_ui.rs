#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;

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
        <div data-rs-hero-media="" class=class>{children()}</div>
    }
}

#[component]
pub fn HeroActions(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-hero-actions="" class=class>{children()}</div>
    }
}

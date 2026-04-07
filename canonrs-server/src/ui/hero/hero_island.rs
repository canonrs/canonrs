use leptos::prelude::*;
use super::hero_ui::{HeroTitle, HeroSubtitle, HeroDescription, HeroMedia, HeroActions};

#[component]
pub fn HeroTitleIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HeroTitle class=class.unwrap_or_default()>{children()}</HeroTitle> }
}

#[component]
pub fn HeroSubtitleIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HeroSubtitle class=class.unwrap_or_default()>{children()}</HeroSubtitle> }
}

#[component]
pub fn HeroDescriptionIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HeroDescription class=class.unwrap_or_default()>{children()}</HeroDescription> }
}

#[component]
pub fn HeroMediaIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HeroMedia class=class.unwrap_or_default()>{children()}</HeroMedia> }
}

#[component]
pub fn HeroActionsIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HeroActions class=class.unwrap_or_default()>{children()}</HeroActions> }
}

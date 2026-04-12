use leptos::prelude::*;
use super::hero_ui::{
    HeroTitle as HeroTitleUi,
    HeroSubtitle as HeroSubtitleUi,
    HeroDescription as HeroDescriptionUi,
    HeroMedia as HeroMediaUi,
    HeroActions as HeroActionsUi
};

#[component]
pub fn HeroTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HeroTitleUi class=class.unwrap_or_default()>{children()}</HeroTitleUi> }
}

#[component]
pub fn HeroSubtitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HeroSubtitleUi class=class.unwrap_or_default()>{children()}</HeroSubtitleUi> }
}

#[component]
pub fn HeroDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HeroDescriptionUi class=class.unwrap_or_default()>{children()}</HeroDescriptionUi> }
}

#[component]
pub fn HeroMedia(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HeroMediaUi class=class.unwrap_or_default()>{children()}</HeroMediaUi> }
}

#[component]
pub fn HeroActions(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <HeroActionsUi class=class.unwrap_or_default()>{children()}</HeroActionsUi> }
}

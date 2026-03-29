//! @canon-id: hero-ui
//! @canon-label: Hero UI
//! @canon-family: display
//! @canon-category: Display
//! @canon-intent: Semantic UI elements inside HeroBlock
//! @canon-description: Hero typography and label components
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: hero, title, subtitle, description, heading

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

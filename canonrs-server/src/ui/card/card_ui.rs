//! @canon-id: card
//! @canon-label: Card
//! @canon-family: layout
//! @canon-category: Display
//! @canon-intent: Group related content in a container
//! @canon-description: Card component
//! @canon-composable: true
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts: CardHeader, CardTitle, CardDescription, CardContent, CardFooter
//! @canon-tags: card, container, group, content

use leptos::prelude::*;
use canonrs_core::primitives::{
    CardPrimitive,
    CardHeaderPrimitive,
    CardTitlePrimitive,
    CardDescriptionPrimitive,
    CardContentPrimitive,
    CardFooterPrimitive,
};

#[component]
pub fn Card(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let base_class = format!("card {}", class);

    view! {
        <CardPrimitive
            class={base_class}
        >
            {children()}
        </CardPrimitive>
    }
}

#[component]
pub fn CardHeader(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CardHeaderPrimitive
            class={class}
        >
            {children()}
        </CardHeaderPrimitive>
    }
}

#[component]
pub fn CardTitle(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CardTitlePrimitive
            class={class}
        >
            {children()}
        </CardTitlePrimitive>
    }
}

#[component]
pub fn CardDescription(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CardDescriptionPrimitive
            class={class}
        >
            {children()}
        </CardDescriptionPrimitive>
    }
}

#[component]
pub fn CardContent(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CardContentPrimitive
            class={class}
        >
            {children()}
        </CardContentPrimitive>
    }
}

#[component]
pub fn CardFooter(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CardFooterPrimitive
            class={class}
        >
            {children()}
        </CardFooterPrimitive>
    }
}

#[component]
pub fn CardPreview() -> impl IntoView {
    view! { <Card>"Card content"</Card> }
}

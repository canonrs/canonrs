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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    let base_class = format!("card {}", class);

    view! {
        <CardPrimitive
            class={base_class}
        >
            {children.map(|c| c())}
        </CardPrimitive>
    }
}

#[component]
pub fn CardHeader(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CardHeaderPrimitive
            class={class}
        >
            {children.map(|c| c())}
        </CardHeaderPrimitive>
    }
}

#[component]
pub fn CardTitle(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CardTitlePrimitive
            class={class}
        >
            {children.map(|c| c())}
        </CardTitlePrimitive>
    }
}

#[component]
pub fn CardDescription(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CardDescriptionPrimitive
            class={class}
        >
            {children.map(|c| c())}
        </CardDescriptionPrimitive>
    }
}

#[component]
pub fn CardContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CardContentPrimitive
            class={class}
        >
            {children.map(|c| c())}
        </CardContentPrimitive>
    }
}

#[component]
pub fn CardFooter(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CardFooterPrimitive
            class={class}
        >
            {children.map(|c| c())}
        </CardFooterPrimitive>
    }
}

#[component]
pub fn CardPreview() -> impl IntoView {
    view! { <Card>"Card content"</Card> }
}

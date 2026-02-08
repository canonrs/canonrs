use leptos::prelude::*;
use crate::primitives::{
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
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let base_class = format!("card {}", class);

    view! {
        <CardPrimitive
            class={base_class}
            id={id}
        >
            {children.map(|c| c())}
        </CardPrimitive>
    }
}

#[component]
pub fn CardHeader(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CardHeaderPrimitive
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </CardHeaderPrimitive>
    }
}

#[component]
pub fn CardTitle(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CardTitlePrimitive
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </CardTitlePrimitive>
    }
}

#[component]
pub fn CardDescription(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CardDescriptionPrimitive
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </CardDescriptionPrimitive>
    }
}

#[component]
pub fn CardContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CardContentPrimitive
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </CardContentPrimitive>
    }
}

#[component]
pub fn CardFooter(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CardFooterPrimitive
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </CardFooterPrimitive>
    }
}

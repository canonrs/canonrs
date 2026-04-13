use leptos::prelude::*;
use super::card_ui::{
    Card as CardUi,
    CardHeader as CardHeaderUi,
    CardTitle as CardTitleUi,
    CardDescription as CardDescriptionUi,
    CardContent as CardContentUi,
    CardFooter as CardFooterUi,
};

#[component]
pub fn Card(
    children: Children,
    #[prop(into, default = String::new())] variant: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CardUi variant=variant class=class>{children()}</CardUi> }
}

#[component]
pub fn CardHeader(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CardHeaderUi class=class>{children()}</CardHeaderUi> }
}

#[component]
pub fn CardTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CardTitleUi class=class>{children()}</CardTitleUi> }
}

#[component]
pub fn CardDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CardDescriptionUi class=class>{children()}</CardDescriptionUi> }
}

#[component]
pub fn CardContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CardContentUi class=class>{children()}</CardContentUi> }
}

#[component]
pub fn CardFooter(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CardFooterUi class=class>{children()}</CardFooterUi> }
}

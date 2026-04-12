use leptos::prelude::*;
use super::card_ui::{
    Card as CardUi,
    CardHeader as CardHeaderUi,
    CardTitle as CardTitleUi,
    CardDescription as CardDescriptionUi,
    CardContent as CardContentUi,
    CardFooter as CardFooterUi
};

#[component]
pub fn Card(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardUi class=cls>
            {children()}
        </CardUi>
    }
}

#[component]
pub fn CardHeader(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardHeaderUi class=cls>
            {children()}
        </CardHeaderUi>
    }
}

#[component]
pub fn CardTitle(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardTitleUi class=cls>
            {children()}
        </CardTitleUi>
    }
}

#[component]
pub fn CardDescription(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardDescriptionUi class=cls>
            {children()}
        </CardDescriptionUi>
    }
}

#[component]
pub fn CardContent(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardContentUi class=cls>
            {children()}
        </CardContentUi>
    }
}

#[component]
pub fn CardFooter(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardFooterUi class=cls>
            {children()}
        </CardFooterUi>
    }
}

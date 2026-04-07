use leptos::prelude::*;
use super::card_ui::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};

#[component]
pub fn CardIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <Card class=cls>
            {children()}
        </Card>
    }
}

#[component]
pub fn CardHeaderIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardHeader class=cls>
            {children()}
        </CardHeader>
    }
}

#[component]
pub fn CardTitleIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardTitle class=cls>
            {children()}
        </CardTitle>
    }
}

#[component]
pub fn CardDescriptionIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardDescription class=cls>
            {children()}
        </CardDescription>
    }
}

#[component]
pub fn CardContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardContent class=cls>
            {children()}
        </CardContent>
    }
}

#[component]
pub fn CardFooterIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let cls = class.unwrap_or_default();
    view! {
        <CardFooter class=cls>
            {children()}
        </CardFooter>
    }
}

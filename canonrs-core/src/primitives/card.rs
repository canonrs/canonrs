//! @canon-level: strict
//! @canon-owner: primitives-team
//! Card Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn CardPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card=""
            data-rs-component="Card" class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card-header="" class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card-title="" class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </div>
    }
}

#[component]
pub fn CardDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card-description="" class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </div>
    }
}

#[component]
pub fn CardContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card-content="" class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooterPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card-footer="" class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </div>
    }
}

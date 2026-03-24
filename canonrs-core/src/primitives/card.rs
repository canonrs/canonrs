//! @canon-level: strict
//! @canon-owner: primitives-team
//! Card Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn CardPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CardHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card-header="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CardTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card-title="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CardDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card-description="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CardContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card-content="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CardFooterPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-rs-card-footer="" class=class id=id>
            {children.map(|c| c())}
        </div>
    }
}

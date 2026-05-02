//! @canon-level: strict
//! @canon-owner: primitives-team
//! Card Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn CardPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] variant: String,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    let uid_crd = crate::infra::uid::generate("crd");
    view! {
        <div
            data-rs-card=""
            data-rs-uid=uid_crd
            data-rs-variant=variant
            role="region"
            aria-label=aria_label
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-card-header="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h3 data-rs-card-title="" class=class>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p data-rs-card-description="" class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-card-content="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooterPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-card-footer="" class=class>
            {children()}
        </div>
    }
}

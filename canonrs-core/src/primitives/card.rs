//! @canon-level: strict
//! @canon-owner: primitives-team
//! Card Primitive - HTML puro

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum CardVariant {
    #[default]
    Default,
    Outlined,
    Elevated,
    Ghost,
}
impl CardVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default  => "default",
            Self::Outlined => "outlined",
            Self::Elevated => "elevated",
            Self::Ghost    => "ghost",
        }
    }
}

#[component]
pub fn CardPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = CardVariant::Default)] variant: CardVariant,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    let uid_crd = crate::infra::uid::generate("crd");
    view! {
        <div
            data-rs-card=""
            data-rs-uid=uid_crd
            data-rs-variant=variant.as_str()
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

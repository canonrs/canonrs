//! @canon-level: strict
//! @canon-owner: primitives-team
//! Logo Primitive - Brand identity anchor

use leptos::prelude::*;

#[component]
pub fn LogoPrimitive(
    children: Children,
    #[prop(into, default = "/".to_string())] href: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = "md".to_string())] size: String,
    #[prop(into, default = "brand".to_string())] variant: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <a
            data-rs-logo=""
            data-rs-component="Logo"
            data-rs-behavior="structural"
            data-rs-size=size
            data-rs-variant=variant
            href=href
            aria-label=aria_label.unwrap_or_default()
            class=class
        >
            {children()}
        </a>
    }
}

#[component]
pub fn LogoIconPrimitive(
    #[prop(into)] src: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <img
            data-rs-logo-icon=""
            src=src
            alt=""
            aria-hidden="true"
            class=class
        />
    }
}

#[component]
pub fn LogoWordmarkPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-logo-wordmark="" class=class>{children()}</span>
    }
}

#[component]
pub fn LogoTaglinePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-logo-tagline="" class=class>{children()}</span>
    }
}

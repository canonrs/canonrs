//! @canon-level: strict
//! @canon-owner: primitives-team
//! Avatar Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn AvatarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-avatar=""
            data-rs-component="Avatar"
            data-rs-behavior="display"
            class=class
        >
            {children()}
        </span>
    }
}

#[component]
pub fn AvatarImagePrimitive(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <img
            data-rs-avatar-image=""
            data-rs-state="visible"
            src=src
            alt=alt
            class=class
        />
    }
}

#[component]
pub fn AvatarFallbackPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-avatar-fallback=""
            data-rs-state="hidden"
            aria-hidden="true"
            class=class
        >
            {children()}
        </span>
    }
}

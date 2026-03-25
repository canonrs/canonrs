//! @canon-level: strict
//! @canon-owner: primitives-team
//! Avatar Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn AvatarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <span data-rs-avatar="" class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </span>
    }
}

#[component]
pub fn AvatarImagePrimitive(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <img
            data-rs-avatar-image=""
            src=src
            alt=alt
            class=class
            id=id.filter(|s| !s.is_empty())
        />
    }
}

#[component]
pub fn AvatarFallbackPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <span data-rs-avatar-fallback="" class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </span>
    }
}

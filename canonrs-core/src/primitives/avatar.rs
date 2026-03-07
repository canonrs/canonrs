//! @canon-level: strict
//! @canon-owner: primitives-team
//! Avatar Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn AvatarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-avatar=""
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn AvatarImagePrimitive(
    src: String,
    alt: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <img
            data-avatar-image=""
            src={src}
            alt={alt}
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        />
    }
}

#[component]
pub fn AvatarFallbackPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-avatar-fallback=""
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </span>
    }
}

//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn AvatarPrimitive(children: Children) -> impl IntoView {
    view! {
        <span data-name="Avatar">
            {children()}
        </span>
    }
}

#[component]
pub fn AvatarImagePrimitive(#[prop(into)] src: String, #[prop(into)] alt: String) -> impl IntoView {
    view! {
        <img data-name="AvatarImage" src=src alt=alt />
    }
}

#[component]
pub fn AvatarFallbackPrimitive(children: Children) -> impl IntoView {
    view! {
        <span data-name="AvatarFallback">
            {children()}
        </span>
    }
}

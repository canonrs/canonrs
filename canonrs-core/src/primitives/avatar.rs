//! @canon-level: strict
//! @canon-owner: primitives-team
//! Avatar Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

#[component]
pub fn AvatarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] size: String,
    #[prop(into, default = String::new())] shape: String,
    #[prop(into, default = String::new())] status: String,
) -> impl IntoView {
    let uid = crate::infra::uid::generate("av");
    view! {
        <span
            data-rs-avatar=""
            data-rs-uid=uid
            data-rs-interaction="init"
            data-rs-size=size
            data-rs-shape=shape
            data-rs-status=status
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
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
) -> impl IntoView {
    let uid = crate::infra::uid::generate("av-img");
    let s = visibility_attrs(state);
    view! {
        <img
            data-rs-avatar-image=""
            data-rs-uid=uid
            data-rs-state=s.data_rs_state
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
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
) -> impl IntoView {
    let uid = crate::infra::uid::generate("av-fb");
    let s = visibility_attrs(state);
    view! {
        <span
            data-rs-avatar-fallback=""
            data-rs-uid=uid
            data-rs-state=s.data_rs_state
            aria-hidden=s.aria_hidden
            class=class
        >
            {children()}
        </span>
    }
}

//! @canon-level: strict
//! @canon-owner: primitives-team
//! Avatar Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;
use crate::primitives::status_dot::{StatusDotPrimitive, StatusDotVariant};

#[component]
pub fn AvatarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] status: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] size: String,
    #[prop(into, default = String::new())] shape: String,
) -> impl IntoView {
    let dot_variant = match status.as_str() {
        "online"  => Some(StatusDotVariant::Online),
        "offline" => Some(StatusDotVariant::Offline),
        "busy"    => Some(StatusDotVariant::Busy),
        "away"    => Some(StatusDotVariant::Away),
        _         => None,
    };
    let status_state = status.clone();
    view! {
        <span
            data-rs-avatar-group=""
            data-rs-uid=crate::infra::uid::generate("av")
            data-rs-interaction="init"
            data-rs-state=status_state
            data-rs-size=size
            class=class
        >
            <span
                data-rs-avatar=""
                data-rs-shape=shape
            >
                {children()}
            </span>
            {dot_variant.map(|v| view! {
                <StatusDotPrimitive variant=v class="">" "</StatusDotPrimitive>
            })}
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
    let s = visibility_attrs(state);
    view! {
        <img
            data-rs-avatar-image=""
            data-rs-uid=crate::infra::uid::generate("av-img")
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
    let s = visibility_attrs(state);
    view! {
        <span
            data-rs-avatar-fallback=""
            data-rs-uid=crate::infra::uid::generate("av-fb")
            data-rs-state=s.data_rs_state
            aria-hidden=s.aria_hidden
            class=class
        >
            {children()}
        </span>
    }
}

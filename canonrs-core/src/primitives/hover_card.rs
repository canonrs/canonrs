//! @canon-level: strict
//! @canon-owner: primitives-team
//! HoverCard Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum HoverCardSide {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl HoverCardSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top    => "top",
            Self::Bottom => "bottom",
            Self::Left   => "left",
            Self::Right  => "right",
        }
    }
}


#[component]
pub fn HoverCardPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_hc = crate::infra::uid::generate("hc");
    view! {
        <div
            data-rs-hover-card=""
            data-rs-uid=uid_hc
            data-rs-interaction="overlay"
            data-rs-visibility=state.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn HoverCardTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-hover-card-trigger=""
            data-rs-visibility=state.as_str()
            aria-expanded=state.aria_expanded()
            tabindex="0"
            class=class
        >
            {children()}
        </span>
    }
}

#[component]
pub fn HoverCardContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = HoverCardSide::Top)] side: HoverCardSide,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-hover-card-content=""
            data-rs-visibility=state.as_str()
            data-rs-side=side.as_str()
            role="tooltip"
            class=class
        >
            {children()}
        </div>
    }
}

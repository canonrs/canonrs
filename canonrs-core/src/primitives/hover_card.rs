//! @canon-level: strict
//! @canon-owner: primitives-team
//! HoverCard Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::state_engine::{visibility_attrs, trigger_attrs};

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
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-hover-card=""
            data-rs-component="HoverCard"
            data-rs-behavior="overlay"
            data-rs-state=s.data_rs_state
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
    let t = trigger_attrs(state);
    view! {
        <span
            data-rs-hover-card-trigger=""
            data-rs-state=t.data_rs_state
            aria-expanded=t.aria_expanded
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
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-hover-card-content=""
            data-rs-state=s.data_rs_state
            data-rs-side=side.as_str()
            role="tooltip"
            aria-hidden=s.aria_hidden
            hidden=s.hidden
            class=class
        >
            {children()}
        </div>
    }
}

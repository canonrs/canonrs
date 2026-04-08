//! @canon-level: strict
//! @canon-owner: primitives-team
//! Sheet Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SheetSide {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}
impl SheetSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Right  => "right",
            Self::Left   => "left",
            Self::Top    => "top",
            Self::Bottom => "bottom",
        }
    }
}

#[component]
pub fn SheetPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = SheetSide::Right)] side: SheetSide,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-sheet=""
            data-rs-interaction="overlay"
            data-rs-component="Sheet"
            data-rs-behavior="overlay"
            data-rs-state=s.data_rs_state
            data-rs-side=side.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SheetTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(optional, into)] aria_controls: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <button
            type="button"
            data-rs-sheet-trigger=""
            data-rs-state=s.data_rs_state
            aria-haspopup="dialog"
            aria-expanded=s.aria_expanded
            aria-controls=aria_controls
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn SheetContentPrimitive(
    children: Children,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional, into)] aria_describedby: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-sheet-content=""
            role="dialog"
            aria-modal="true"
            aria-labelledby=aria_labelledby
            aria-describedby=aria_describedby
            tabindex="-1"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SheetOverlayPrimitive(
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-sheet-overlay=""
            data-rs-state=s.data_rs_state
            class=class
        />
    }
}

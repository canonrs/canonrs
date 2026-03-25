//! @canon-level: strict
//! @canon-owner: primitives-team
//! Sheet Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::state_engine::visibility_attrs;

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
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = SheetSide::Right)] side: SheetSide,
    #[prop(default = VisibilityState::Closed)] open: VisibilityState,
) -> impl IntoView {
    view! {
        <div
            data-rs-sheet=""
            data-rs-component="Sheet"
            data-rs-behavior="overlay"
            data-rs-state={visibility_attrs(open).data_rs_state}
            data-rs-side=side.as_str()
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SheetTriggerPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-sheet-trigger=""
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn SheetContentPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-sheet-content=""
            role="dialog"
            aria-modal="true"
            tabindex="-1"
            class=class
            id=if id.is_empty() { None } else { Some(id.clone()) }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SheetOverlayPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-rs-sheet-overlay="" aria-hidden="true" class=class id=if id.is_empty() { None } else { Some(id.clone()) } />
    }
}

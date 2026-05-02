//! @canon-level: strict
//! @canon-owner: primitives-team
//! Drawer Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum DrawerSide {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}
impl DrawerSide {
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
pub fn DrawerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = DrawerSide::Right)] side: DrawerSide,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_dr = crate::infra::uid::generate("dr");
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-drawer=""
            data-rs-interaction="overlay"
            data-rs-uid=uid_dr
            data-rs-state=s.data_rs_state
            data-rs-side=side.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(optional, into)] aria_controls: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <button
            type="button"
            data-rs-drawer-trigger=""
            data-rs-button=""
            data-rs-variant="primary"
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
pub fn DrawerOverlayPrimitive(
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-drawer-overlay=""
            data-rs-state=s.data_rs_state
            class=class
        />
    }
}

#[component]
pub fn DrawerContentPrimitive(
    children: Children,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional, into)] aria_describedby: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-drawer-content=""
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
pub fn DrawerPortalPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <div data-rs-drawer-portal="" class=class>{children()}</div> }
}

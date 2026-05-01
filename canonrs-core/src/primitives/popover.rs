//! @canon-level: strict
//! @canon-owner: primitives-team
//! Popover Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::{visibility_attrs, trigger_attrs};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PopoverSide {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

impl PopoverSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bottom => "bottom",
            Self::Top    => "top",
            Self::Left   => "left",
            Self::Right  => "right",
        }
    }
}


#[component]
pub fn PopoverPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] name: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-popover=""
            data-rs-uid=crate::infra::uid::generate("pop")
            data-rs-interaction="overlay"
            data-rs-state=s.data_rs_state
            data-rs-name=name
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PopoverTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] label: String,
) -> impl IntoView {
    let t = trigger_attrs(state);
    view! {
        <button
            type="button"
            data-rs-popover-trigger=""
            data-rs-button=""
            data-rs-variant="outline"
            data-rs-state=t.data_rs_state
            data-rs-value=value
            data-rs-label=label
            aria-haspopup="dialog"
            aria-expanded=t.aria_expanded
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn PopoverContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = PopoverSide::Bottom)] side: PopoverSide,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-popover-content=""
            data-rs-state=s.data_rs_state
            data-rs-side=side.as_str()
            role="dialog"
            aria-modal="false"
            aria-label=aria_label
            tabindex="-1"
            class=class
        >
            {children()}
        </div>
    }
}

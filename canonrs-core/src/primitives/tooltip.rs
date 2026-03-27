//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tooltip Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::visibility_attrs;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TooltipSide {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[component]
pub fn TooltipPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-tooltip=""
            data-rs-component="Tooltip"
            data-rs-behavior="overlay"
            data-rs-state=s.data_rs_state
            class=class
        >
            {children()}
        </div>
    }
}

// Trigger é span — tooltip funciona em qualquer elemento
#[component]
pub fn TooltipTriggerPrimitive(
    children: Children,
    #[prop(into, optional)] tooltip_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-tooltip-trigger=""
            aria-describedby=tooltip_id
            tabindex="0"
            class=class
        >
            {children()}
        </span>
    }
}

#[component]
pub fn TooltipContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(default = TooltipSide::Top)] side: TooltipSide,
    #[prop(into, optional)] tooltip_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-tooltip-content=""
            data-rs-state=s.data_rs_state
            data-rs-side=side.as_str()
            id=tooltip_id
            role="tooltip"
            aria-hidden=s.aria_hidden
            hidden=s.hidden
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TooltipProviderPrimitive(
    children: Children,
    #[prop(default = 400)] delay_open: u32,
    #[prop(default = 100)] delay_close: u32,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tooltip-provider=""
            data-rs-delay-open=delay_open.to_string()
            data-rs-delay-close=delay_close.to_string()
            class=class
        >
            {children()}
        </div>
    }
}

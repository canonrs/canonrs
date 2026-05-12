//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tooltip Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;

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
    let uid_tip = crate::infra::uid::generate("tip");
    view! {
        <div
            data-rs-tooltip=""
            data-rs-uid=uid_tip
            data-rs-interaction="init"
            data-rs-visibility=state.as_str()
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
    #[prop(default = true)] arrow: bool,
    #[prop(into, optional)] tooltip_id: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tooltip-content=""
            data-rs-visibility=state.as_str()
            data-rs-side=side.as_str()
            id=tooltip_id
            role="tooltip"
            class=class
        >
            {children()}
            {arrow.then(|| view! { <span data-rs-tooltip-arrow="" /> })}
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

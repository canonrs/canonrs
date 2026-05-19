//! @canon-level: strict
//! @canon-owner: primitives-team
//! Popover Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;

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
    let uid_pop = crate::infra::uid::generate("pop");
    view! {
        <div
            data-rs-popover=""
            data-rs-uid=uid_pop
            data-rs-interaction="overlay"
            data-rs-visibility=state.as_str()
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
    view! {
        <button
            type="button"
            data-rs-popover-trigger=""
            data-rs-button=""
            data-rs-variant="outline"
            data-rs-visibility=state.as_str()
            data-rs-value=value
            data-rs-label=label
            aria-haspopup="dialog"
            aria-expanded=state.aria_expanded()
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
    view! {
        <div
            data-rs-popover-content=""
            data-rs-visibility=state.as_str()
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

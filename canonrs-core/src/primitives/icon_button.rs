//! @canon-level: strict
//! @canon-owner: primitives-team
//! IconButton Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::{DisabledState, LoadingState};
use crate::state_engine::{disabled_attrs, loading_attrs};

#[derive(Clone, PartialEq, Default, Debug)]
pub enum IconButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}
impl IconButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub enum IconButtonVariant {
    #[default]
    Default,
    Ghost,
    Outline,
    Destructive,
}
impl IconButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Ghost       => "ghost",
            Self::Outline     => "outline",
            Self::Destructive => "destructive",
        }
    }
}

#[component]
pub fn IconButtonPrimitive(
    children: Children,
    #[prop(into)] aria_label: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = LoadingState::Idle)] loading: LoadingState,
    #[prop(default = IconButtonVariant::Default)] variant: IconButtonVariant,
    #[prop(default = IconButtonSize::Md)] size: IconButtonSize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d  = disabled_attrs(disabled);
    let la = loading_attrs(loading);
    view! {
        <button
            type="button"
            data-rs-icon-button=""
            data-rs-component="IconButton"
            data-rs-behavior="action"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            data-rs-disabled=d.data_rs_disabled
            data-rs-loading=la.data_rs_state
            disabled=d.disabled
            aria-disabled=d.aria_disabled
            aria-busy=la.aria_busy
            aria-label=aria_label
            class=class
        >
            {children()}
        </button>
    }
}

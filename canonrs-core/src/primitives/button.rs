//! @canon-level: strict
//! @canon-owner: primitives-team
//! Button Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::{DisabledState, LoadingState, ToggleState};
use crate::infra::state_engine::{disabled_attrs, loading_attrs, toggle_attrs};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
    Primary,
}
impl ButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Destructive => "destructive",
            Self::Outline     => "outline",
            Self::Secondary   => "secondary",
            Self::Ghost       => "ghost",
            Self::Link        => "link",
            Self::Primary     => "primary",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Icon,
}
impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Xs   => "xs",
            Self::Sm   => "sm",
            Self::Md   => "md",
            Self::Lg   => "lg",
            Self::Xl   => "xl",
            Self::Icon => "icon",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Serialize, serde::Deserialize)]
pub enum ButtonStateHint {
    #[default] None,
    First, Last, Hover, Focus,
}

impl ButtonStateHint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None  => "",
            Self::First => "first",
            Self::Last  => "last",
            Self::Hover => "hover",
            Self::Focus => "focus",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Serialize, serde::Deserialize)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}
impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset  => "reset",
        }
    }
}

#[component]
pub fn ButtonPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = ButtonType::Button)] button_type: ButtonType,
    #[prop(default = LoadingState::Idle)] loading: LoadingState,
    #[prop(optional)] pressed: Option<ToggleState>,
) -> impl IntoView {
    let d  = disabled_attrs(disabled);
    let la = loading_attrs(loading);
    let ta = pressed.map(toggle_attrs);
    view! {
        <button
            type=button_type.as_str()
            data-rs-button=""
            data-rs-uid=crate::infra::uid::generate("bt")
            data-rs-interaction="init"
            data-rs-component="Button"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            data-rs-disabled=d.data_rs_disabled
            data-rs-loading=la.data_rs_state
            data-rs-state=if d.disabled { Some("disabled") } else { ta.as_ref().map(|t| t.data_rs_state) }
            disabled=d.disabled
            aria-disabled=d.aria_disabled
            aria-busy=la.aria_busy
            aria-pressed=ta.as_ref().map(|t| t.aria_pressed)
            aria-label=aria_label
            class=class
        >
            {children()}
        </button>
    }
}

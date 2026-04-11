//! @canon-level: strict
//! @canon-owner: primitives-team
//! IconButton Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::{DisabledState, LoadingState, ToggleState};
use crate::infra::state_engine::{disabled_attrs, loading_attrs, toggle_attrs};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default, Debug)]
pub enum IconButtonSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}
impl IconButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default, Debug)]
pub enum IconButtonVariant {
    #[default]
    Default,
    Ghost,
    Outline,
    Solid,
    Subtle,
    Destructive,
}
impl IconButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Ghost       => "ghost",
            Self::Outline     => "outline",
            Self::Solid       => "solid",
            Self::Subtle      => "subtle",
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
    #[prop(optional)] pressed: Option<ToggleState>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let d  = disabled_attrs(disabled);
    let la = loading_attrs(loading);
    let ta = pressed.map(toggle_attrs);
    view! {
        <button
            type="button"
            data-rs-icon-button=""
            data-rs-uid=crate::infra::uid::generate("ib")
            data-rs-interaction="init"
            data-rs-component="IconButton"
            data-rs-behavior="icon-button"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            data-rs-disabled=d.data_rs_disabled
            data-rs-loading=la.data_rs_state
            data-rs-state=if d.disabled { Some("disabled") } else if la.aria_busy.is_some() { Some("loading") } else { ta.as_ref().map(|t| t.data_rs_state) }
            disabled=d.disabled
            aria-disabled=d.aria_disabled
            aria-busy=la.aria_busy
            aria-pressed=ta.as_ref().map(|t| t.aria_pressed)
            aria-label=aria_label
            class=class
        >
            <span data-rs-icon-button-inner="">{children()}</span>
        </button>
    }
}

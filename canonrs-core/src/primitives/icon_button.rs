//! @canon-level: strict
//! @canon-owner: primitives-team
//! IconButton Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::{DisabledState, LoadingState, ToggleState};

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
    let uid_ib = crate::infra::uid::generate("ib");
    let ta = pressed;
    view! {
        <button
            type="button"
            data-rs-icon-button=""
            data-rs-uid=uid_ib
            data-rs-interaction="init"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            data-rs-loading=loading.as_str()
            data-rs-toggle=ta.map(|t| t.as_str())
            disabled=disabled.disabled()
            aria-disabled=disabled.aria_disabled()
            aria-busy=loading.aria_busy()
            aria-pressed=ta.as_ref().map(|t| t.aria_pressed())
            aria-label=aria_label
            class=class
        >
            <span data-rs-icon-button-inner="">{children()}</span>
        </button>
    }
}

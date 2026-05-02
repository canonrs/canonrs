//! @canon-level: strict
//! @canon-owner: primitives-team
//! Button Primitive - HTML puro
use leptos::prelude::*;
use crate::meta::{DisabledState, LoadingState, ToggleState};
use crate::infra::state_engine::{UiStateAttrs};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ButtonVariant {
    #[default]
    Default, Destructive, Outline, Secondary, Ghost, Link, Primary,
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
    Xs, Sm,
    #[default]
    Md,
    Lg, Xl, Icon,
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
    Button, Submit, Reset,
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
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = ButtonType::Button)] button_type: ButtonType,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = LoadingState::Idle)] loading: LoadingState,
    #[prop(optional)] pressed: Option<ToggleState>,
) -> impl IntoView {
    let uid   = crate::infra::uid::generate("bt");
    let attrs = UiStateAttrs::from_button(disabled, loading, pressed);
    view! {
        <button
            type=button_type.as_str()
            data-rs-button=""
            data-rs-uid=uid
            data-rs-interaction="init"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            data-rs-disabled=attrs.data_rs_disabled
            data-rs-loading=attrs.data_rs_loading
            data-rs-state=attrs.data_rs_state
            disabled=attrs.disabled
            aria-disabled=attrs.aria_disabled
            aria-busy=attrs.aria_busy
            aria-pressed=attrs.aria_pressed
            aria-label=aria_label
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn LinkButtonPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] target: String,
) -> impl IntoView {
    let uid   = crate::infra::uid::generate("bt");
    let attrs = UiStateAttrs::from_button(disabled, LoadingState::Idle, None);
    view! {
        <a
            href=href
            target=if target.is_empty() { None } else { Some(target) }
            data-rs-button=""
            data-rs-uid=uid
            data-rs-interaction="init"
            data-rs-variant=variant.as_str()
            data-rs-size=size.as_str()
            data-rs-disabled=attrs.data_rs_disabled
            aria-disabled=attrs.aria_disabled
            aria-label=aria_label
            class=class
        >
            {children()}
        </a>
    }
}

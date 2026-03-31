
use leptos::prelude::*;
use canonrs_core::meta::{DisabledState, LoadingState};
use crate::ui::icon::IconSize;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IconButtonVariant { Default, Ghost, Outline, Solid, Destructive }
impl IconButtonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Ghost => "ghost",
            Self::Outline => "outline",
            Self::Solid => "solid",
            Self::Destructive => "destructive",
        }
    }
}

#[component]
pub fn IconButton(
    children: Children,
    aria_label: String,
    #[prop(default = IconSize::Md)] size: IconSize,
    #[prop(default = IconButtonVariant::Default)] variant: IconButtonVariant,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = LoadingState::Idle)] loading: LoadingState,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let is_loading = loading == LoadingState::Loading;
    let is_disabled = disabled == DisabledState::Disabled || is_loading;
    view! {
        <button
            data-rs-icon-button=""
            data-rs-size={size.as_str()}
            data-rs-variant={variant.as_str()}
            data-rs-loading={if is_loading { Some("") } else { None }}
            class={class}
            id={id}
            disabled=is_disabled
            aria-label={aria_label}
            aria-busy={if is_loading { Some("true") } else { None }}
        >
            {children()}
        </button>
    }
}

#[component]
pub fn IconButtonPreview() -> impl IntoView {
    view! { <IconButton aria_label="Close".to_string()>"×"</IconButton> }
}

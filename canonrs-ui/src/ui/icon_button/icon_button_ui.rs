//! IconButton UI Component - Enterprise Action Button
//! Composes ButtonPrimitive with Icon wrapper

use leptos::prelude::*;
use crate::ui::icon::{Icon, IconSize};

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
    aria_label: String, // OBRIGATÓRIO - acessibilidade crítica
    #[prop(default = IconSize::Md)] size: IconSize,
    #[prop(default = IconButtonVariant::Default)] variant: IconButtonVariant,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] loading: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let content = if loading {
        view! { "⏳" }.into_any()
    } else {
        children().into_any()
    };

    view! {
        <button
            data-icon-button=""
            data-size={size.as_str()}
            data-variant={variant.as_str()}
            data-loading={loading.then_some("")}
            class={class}
            id={id}
            disabled={disabled || loading}
            aria-label={aria_label}
            aria-busy={loading.then_some("true")}
        >
            <Icon size={size} spin=loading>
                {content}
            </Icon>
        </button>
    }
}

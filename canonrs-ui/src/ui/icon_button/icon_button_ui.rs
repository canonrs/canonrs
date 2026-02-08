//! IconButton UI Component
//! Composes ButtonPrimitive with Icon wrapper

use leptos::prelude::*;
use crate::primitives::ButtonPrimitive;
use crate::ui::icon::Icon;

#[component]
pub fn IconButton(
    children: Children,
    #[prop(default = "md")] size: &'static str,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] data_behavior: Option<String>,
) -> impl IntoView {
    view! {
        <ButtonPrimitive
            attr:data-icon-button=""
            attr:data-size={size}
            disabled={disabled}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
            aria_label={aria_label.unwrap_or_default()}
            attr:data-behavior={data_behavior.unwrap_or_default()}
        >
            <Icon size={size}>
                {children()}
            </Icon>
        </ButtonPrimitive>
    }
}

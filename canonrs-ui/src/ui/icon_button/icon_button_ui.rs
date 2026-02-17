//! IconButton UI Component
//! Composes ButtonPrimitive with Icon wrapper

use leptos::prelude::*;
use crate::primitives::ButtonPrimitive;
use crate::ui::icon::{Icon, IconSize};

#[component]
pub fn IconButton(
    children: Children,
    #[prop(default = IconSize::Md)] size: IconSize,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] data_behavior: Option<String>,
) -> impl IntoView {
    let size_str = size.as_str();
    view! {
        <ButtonPrimitive
            class={format!("icon-button-size-{} {}", size_str, class.unwrap_or_default())}
            id={id.unwrap_or_default()}
            disabled={disabled}
        >
            <Icon size={size}>
                {children()}
            </Icon>
        </ButtonPrimitive>
    }
}

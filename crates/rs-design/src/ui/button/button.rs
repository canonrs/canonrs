use crate::primitives::button::*;
use leptos::prelude::*;

use super::types::ButtonType;
use super::variants::{ButtonVariant, ButtonSize, ValidationState};

const BASE_CLASSES: &str = "\
    inline-flex items-center justify-center \
    font-medium \
    rounded-md \
    border \
    shadow-sm \
    transition-all \
    focus-visible:outline-none \
    disabled:pointer-events-none";

#[component]
pub fn Button(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] validation: Option<ValidationState>,
    #[prop(default = ButtonType::Submit)] button_type: ButtonType,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let ui_variant = variant.unwrap_or_default();
    let ui_size = size.unwrap_or_default();

    let validation_classes = validation.map(|v| v.classes()).unwrap_or("");

    let classes = format!(
        "{} {} {} {} {}",
        BASE_CLASSES,
        ui_size.classes(),
        ui_variant.classes(),
        validation_classes,
        class
    );
    
    let inline_style = format!(
        "font-family: var(--font-family-sans); \
         z-index: var(--z-base); \
         line-height: var(--line-height-normal); \
         {}",
        ui_size.style()
    );

    view! {
        <ButtonPrimitive
            disabled=disabled
            button_type=button_type.as_str()
            class=classes
            attr:style=inline_style
        >
            {children()}
        </ButtonPrimitive>
    }
}

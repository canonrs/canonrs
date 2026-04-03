use leptos::prelude::*;
use canonrs_core::meta::{DisabledState, LoadingState, ToggleState};
use canonrs_core::primitives::IconButtonPrimitive;
pub use canonrs_core::primitives::{IconButtonSize, IconButtonVariant};

#[component]
pub fn IconButton(
    children: Children,
    aria_label: String,
    #[prop(default = IconButtonSize::Md)] size: IconButtonSize,
    #[prop(default = IconButtonVariant::Default)] variant: IconButtonVariant,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(default = LoadingState::Idle)] loading: LoadingState,
    #[prop(optional)] pressed: Option<ToggleState>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <IconButtonPrimitive
            aria_label=aria_label
            size=size
            variant=variant
            disabled=disabled
            loading=loading
            pressed=pressed.unwrap_or_default()
            class=class
        >
            {children()}
        </IconButtonPrimitive>
    }
}

#[component]
pub fn IconButtonPreview() -> impl IntoView {
    view! { <IconButton aria_label="Close".to_string() variant=IconButtonVariant::Solid>"×"</IconButton> }
}

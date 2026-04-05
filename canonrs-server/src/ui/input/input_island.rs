use leptos::prelude::*;
use canonrs_core::primitives::{InputPrimitive, InputVariant, InputSize};
use canonrs_core::meta::DisabledState;

#[island]
pub fn InputIsland(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] input_type: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional, into)] size: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let variant = match variant.as_deref() {
        Some("error")   => InputVariant::Error,
        Some("success") => InputVariant::Success,
        Some("warning") => InputVariant::Warning,
        _               => InputVariant::Default,
    };

    let size = match size.as_deref() {
        Some("sm") => InputSize::Sm,
        Some("lg") => InputSize::Lg,
        _          => InputSize::Md,
    };

    let disabled = if disabled.unwrap_or(false) {
        DisabledState::Disabled
    } else {
        DisabledState::Enabled
    };

    view! {
        <InputPrimitive
            class=class.unwrap_or_default()
            input_type=input_type.unwrap_or_else(|| "text".to_string())
            name=name.unwrap_or_default()
            value=value.unwrap_or_default()
            disabled=disabled
            variant=variant
            size=size
            placeholder=placeholder.unwrap_or_default()
            aria_label=aria_label.unwrap_or_default()
        />
    }
}

use leptos::prelude::*;
use super::icon_ui::{Icon, IconSize, IconVariant};

#[island]
pub fn IconIsland(
    children: Children,
    #[prop(optional, into)] size: Option<String>,
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional)] spin: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let size = match size.as_deref() {
        Some("sm") => IconSize::Sm,
        Some("lg") => IconSize::Lg,
        _          => IconSize::Md,
    };
    let variant = match variant.as_deref() {
        Some("muted")       => IconVariant::Muted,
        Some("primary")     => IconVariant::Primary,
        Some("destructive") => IconVariant::Destructive,
        Some("success")     => IconVariant::Success,
        Some("warning")     => IconVariant::Warning,
        _                   => IconVariant::Default,
    };
    let spin = spin.unwrap_or(false);
    let cls = class.unwrap_or_default();
    let id = id.unwrap_or_default();

    view! {
        <Icon size=size variant=variant spin=spin class=cls id=id>
            {children()}
        </Icon>
    }
}

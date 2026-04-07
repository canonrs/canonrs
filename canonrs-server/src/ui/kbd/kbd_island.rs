use leptos::prelude::*;
use super::kbd_ui::{Kbd, KbdGroup, KbdSeparator, KbdSize, KbdVariant};

#[component]
pub fn KbdIsland(
    children: Children,
    #[prop(optional, into)] size: Option<String>,
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let size_val = match size.as_deref() {
        Some("sm") => KbdSize::Sm,
        Some("lg") => KbdSize::Lg,
        _          => KbdSize::Md,
    };
    let variant_val = match variant.as_deref() {
        Some("outline") => KbdVariant::Outline,
        Some("ghost")   => KbdVariant::Ghost,
        _               => KbdVariant::Default,
    };
    view! {
        <Kbd size=size_val variant=variant_val class=class.unwrap_or_default()>
            {children()}
        </Kbd>
    }
}

#[component]
pub fn KbdGroupIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <KbdGroup class=class.unwrap_or_default()>{children()}</KbdGroup> }
}

#[component]
pub fn KbdSeparatorIsland() -> impl IntoView {
    view! { <KbdSeparator /> }
}

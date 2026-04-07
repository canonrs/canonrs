use leptos::prelude::*;
use super::logo_ui::{Logo, LogoSize, LogoVariant};

#[component]
pub fn LogoIsland(
    #[prop(optional, into)] size: Option<String>,
    #[prop(optional, into)] variant: Option<String>,
    #[prop(optional)] wordmark: Option<ChildrenFn>,
    #[prop(optional)] tagline: Option<ChildrenFn>,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let size_val = match size.as_deref() {
        Some("sm") => LogoSize::Sm,
        Some("lg") => LogoSize::Lg,
        _          => LogoSize::Md,
    };
    let variant_val = match variant.as_deref() {
        Some("neutral") => LogoVariant::Neutral,
        _               => LogoVariant::Brand,
    };
    let href_val   = href.unwrap_or_else(|| "/".to_string());
    let aria_val   = aria_label.unwrap_or_default();
    let class_val  = class.unwrap_or_default();

    // Leptos #[prop(optional)] ChildrenFn nao aceita Option no view! — usa match
    match (wordmark, tagline) {
        (Some(wm), Some(tl)) => view! {
            <Logo size=size_val variant=variant_val wordmark=wm tagline=tl href=href_val aria_label=aria_val class=class_val />
        }.into_any(),
        (Some(wm), None) => view! {
            <Logo size=size_val variant=variant_val wordmark=wm href=href_val aria_label=aria_val class=class_val />
        }.into_any(),
        (None, Some(tl)) => view! {
            <Logo size=size_val variant=variant_val tagline=tl href=href_val aria_label=aria_val class=class_val />
        }.into_any(),
        (None, None) => view! {
            <Logo size=size_val variant=variant_val href=href_val aria_label=aria_val class=class_val />
        }.into_any(),
    }
}

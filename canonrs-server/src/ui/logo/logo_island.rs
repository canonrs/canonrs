//! Logo Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.
//! Note: match on wordmark/tagline is structural (Option<ChildrenFn> Leptos constraint), not logic.

use leptos::prelude::*;
use super::logo_ui::{Logo, LogoSize, LogoVariant};

#[component]
pub fn LogoIsland(
    #[prop(default = LogoSize::Md)] size:          LogoSize,
    #[prop(default = LogoVariant::Brand)] variant: LogoVariant,
    #[prop(optional)] wordmark:                    Option<ChildrenFn>,
    #[prop(optional)] tagline:                     Option<ChildrenFn>,
    #[prop(into, default = "/".to_string())] href: String,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(into, default = String::new())] class:  String,
) -> impl IntoView {
    match (wordmark, tagline) {
        (Some(wm), Some(tl)) => view! {
            <Logo size=size variant=variant wordmark=wm tagline=tl href=href aria_label=aria_label class=class />
        }.into_any(),
        (Some(wm), None) => view! {
            <Logo size=size variant=variant wordmark=wm href=href aria_label=aria_label class=class />
        }.into_any(),
        (None, Some(tl)) => view! {
            <Logo size=size variant=variant tagline=tl href=href aria_label=aria_label class=class />
        }.into_any(),
        (None, None) => view! {
            <Logo size=size variant=variant href=href aria_label=aria_label class=class />
        }.into_any(),
    }
}

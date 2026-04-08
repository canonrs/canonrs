//! Kbd Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::kbd_ui::{Kbd, KbdGroup, KbdSeparator, KbdSize, KbdVariant};

#[component]
pub fn KbdIsland(
    children: Children,
    #[prop(default = KbdSize::Md)] size:          KbdSize,
    #[prop(default = KbdVariant::Default)] variant: KbdVariant,
    #[prop(into, default = String::new())] class:  String,
) -> impl IntoView {
    view! { <Kbd size=size variant=variant class=class>{children()}</Kbd> }
}

#[component]
pub fn KbdGroupIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <KbdGroup class=class>{children()}</KbdGroup> }
}

#[component]
pub fn KbdSeparatorIsland() -> impl IntoView {
    view! { <KbdSeparator /> }
}

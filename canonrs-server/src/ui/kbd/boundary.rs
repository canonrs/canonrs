//! Kbd Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::kbd_ui::{
    KbdSize,
    KbdVariant
};

#[component]
pub fn Kbd(
    children: Children,
    #[prop(default = KbdSize::Md)] size:          KbdSize,
    #[prop(default = KbdVariant::Default)] variant: KbdVariant,
    #[prop(into, default = String::new())] class:  String,
) -> impl IntoView {
    view! { <super::kbd_ui::Kbd size=size variant=variant class=class>{children()}</super::kbd_ui::Kbd> }
}

#[component]
pub fn KbdGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <super::kbd_ui::KbdGroup class=class>{children()}</super::kbd_ui::KbdGroup> }
}

#[component]
pub fn KbdSeparator() -> impl IntoView {
    view! { <super::kbd_ui::KbdSeparator /> }
}

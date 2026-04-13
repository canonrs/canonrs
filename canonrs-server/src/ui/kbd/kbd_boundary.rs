//! Kbd Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
pub use super::kbd_ui::{KbdSize, KbdVariant};
use super::kbd_ui;

#[component]
pub fn Kbd(
    children: Children,
    #[prop(default = KbdSize::Md)] size:          KbdSize,
    #[prop(default = KbdVariant::Default)] variant: KbdVariant,
    #[prop(into, default = String::new())] class:  String,
) -> impl IntoView {
    view! { <kbd_ui::Kbd size=size variant=variant class=class>{children()}</kbd_ui::Kbd> }
}

#[component]
pub fn KbdGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <kbd_ui::KbdGroup class=class>{children()}</kbd_ui::KbdGroup> }
}

#[component]
pub fn KbdSeparator() -> impl IntoView {
    view! { <kbd_ui::KbdSeparator /> }
}

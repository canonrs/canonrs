//! @canon-id: kbd
//! @canon-label: Kbd
//! @canon-family: typography
//! @canon-category: Display
//! @canon-intent: Display keyboard shortcut
//! @canon-description: Keyboard shortcut display
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: kbd, keyboard, shortcut, ctrl, cmd

use leptos::prelude::*;
use canonrs_core::primitives::{KbdPrimitive, KbdGroupPrimitive, KbdSeparatorPrimitive};
pub use canonrs_core::primitives::{KbdSize, KbdVariant};

#[component]
pub fn Kbd(
    children: Children,
    #[prop(default = KbdSize::Md)] size: KbdSize,
    #[prop(default = KbdVariant::Default)] variant: KbdVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <KbdPrimitive size=size variant=variant class=class>
            {children()}
        </KbdPrimitive>
    }
}

#[component]
pub fn KbdGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <KbdGroupPrimitive class=class>
            {children()}
        </KbdGroupPrimitive>
    }
}

#[component]
pub fn KbdSeparator() -> impl IntoView {
    view! { <KbdSeparatorPrimitive /> }
}

#[component]
pub fn KbdPreview() -> impl IntoView {
    view! {
        <KbdGroup>
            <Kbd>"Ctrl"</Kbd>
            <KbdSeparator />
            <Kbd>"K"</Kbd>
        </KbdGroup>
    }
}

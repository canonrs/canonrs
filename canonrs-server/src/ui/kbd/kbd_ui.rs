//! @canon-level: ui
//! Kbd - sem behavior

use leptos::prelude::*;
use canonrs_core::primitives::{KbdPrimitive, KbdGroupPrimitive, KbdSeparatorPrimitive};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum KbdSize { Sm, Md }
impl KbdSize {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Sm => "sm", Self::Md => "md" }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum KbdVariant { Default, Muted }
impl KbdVariant {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Default => "default", Self::Muted => "muted" }
    }
}

#[component]
pub fn Kbd(
    children: Children,
    #[prop(default = KbdSize::Md)] size: KbdSize,
    #[prop(default = KbdVariant::Default)] variant: KbdVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <KbdPrimitive
            size=size.as_str().to_string()
            variant=variant.as_str().to_string()
            class=class
        >
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

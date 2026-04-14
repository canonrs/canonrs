//! @canon-level: strict
//! @canon-owner: primitives-team
//! Kbd Primitive - HTML puro

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum KbdSize {
    Sm,
    #[default]
    Md,
    Lg,
}
impl KbdSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum KbdVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Muted,
}
impl KbdVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
            Self::Ghost   => "ghost",
            Self::Muted   => "muted",
        }
    }
}

#[component]
pub fn KbdPrimitive(
    children: Children,
    #[prop(default = KbdSize::Md)] size: KbdSize,
    #[prop(default = KbdVariant::Default)] variant: KbdVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <kbd
            data-rs-kbd=""
            data-rs-uid=crate::infra::uid::generate("kbd")
            data-rs-size=size.as_str()
            data-rs-variant=variant.as_str()
            class=class
        >
            {children()}
        </kbd>
    }
}

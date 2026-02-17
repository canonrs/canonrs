use leptos::prelude::*;
use crate::primitives::{KbdGroupPrimitive, KbdSeparatorPrimitive};

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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = KbdSize::Md)] size: KbdSize,
    #[prop(default = KbdVariant::Default)] variant: KbdVariant,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <kbd
            data-kbd=""
            data-size={size.as_str()}
            data-variant={variant.as_str()}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </kbd>
    }
}

#[component]
pub fn KbdGroup(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <KbdGroupPrimitive class={class} id={id.unwrap_or_default()}>
            {children.map(|c| c())}
        </KbdGroupPrimitive>
    }
}

#[component]
pub fn KbdSeparator() -> impl IntoView {
    view! {
        <KbdSeparatorPrimitive />
    }
}

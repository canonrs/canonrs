use leptos::prelude::*;
use crate::primitives::{AvatarPrimitive, AvatarImagePrimitive, AvatarFallbackPrimitive};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AvatarSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl AvatarSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            AvatarSize::Xs => "xs",
            AvatarSize::Sm => "sm",
            AvatarSize::Md => "md",
            AvatarSize::Lg => "lg",
            AvatarSize::Xl => "xl",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AvatarShape {
    Circle,
    Square,
    Rounded,
}

impl AvatarShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            AvatarShape::Circle => "circle",
            AvatarShape::Square => "square",
            AvatarShape::Rounded => "rounded",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AvatarStatus {
    Online,
    Offline,
    Busy,
    Away,
}

impl AvatarStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AvatarStatus::Online => "online",
            AvatarStatus::Offline => "offline",
            AvatarStatus::Busy => "busy",
            AvatarStatus::Away => "away",
        }
    }
}

#[component]
pub fn Avatar(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = AvatarSize::Md)] size: AvatarSize,
    #[prop(default = AvatarShape::Circle)] shape: AvatarShape,
    #[prop(optional)] status: Option<AvatarStatus>,
    #[prop(optional)] badge: Option<i32>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AvatarPrimitive
            class={format!("{} avatar-size-{} avatar-shape-{}", class, size.as_str(), shape.as_str())}
            id={id.unwrap_or_default()}
        >
            {children.map(|c| c())}
            {status.map(|s| view! {
                <span data-avatar-status="" data-status={s.as_str()}></span>
            })}
            {badge.map(|b| view! {
                <span data-avatar-badge="">{b}</span>
            })}
        </AvatarPrimitive>
    }
}

#[component]
pub fn AvatarImage(
    src: String,
    alt: String,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AvatarImagePrimitive
            src={src}
            alt={alt}
            class={class}
            id={id.unwrap_or_default()}
        />
    }
}

#[component]
pub fn AvatarFallback(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AvatarFallbackPrimitive
            class={class}
            id={id.unwrap_or_default()}
        >
            {children.map(|c| c())}
        </AvatarFallbackPrimitive>
    }
}

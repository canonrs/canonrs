#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{AvatarPrimitive, AvatarImagePrimitive, AvatarFallbackPrimitive, StatusDotPrimitive, StatusDotVariant};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AvatarSize { Xs, Sm, Md, Lg, Xl }
impl AvatarSize {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Xs=>"xs", Self::Sm=>"sm", Self::Md=>"md", Self::Lg=>"lg", Self::Xl=>"xl" }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AvatarShape { Circle, Square, Rounded }
impl AvatarShape {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Circle=>"circle", Self::Square=>"square", Self::Rounded=>"rounded" }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AvatarStatus { Online, Offline, Busy, Away }
impl AvatarStatus {
    pub fn to_variant(&self) -> StatusDotVariant {
        match self {
            Self::Online  => StatusDotVariant::Online,
            Self::Offline => StatusDotVariant::Offline,
            Self::Busy    => StatusDotVariant::Busy,
            Self::Away    => StatusDotVariant::Away,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Online  => "online",
            Self::Offline => "offline",
            Self::Busy    => "busy",
            Self::Away    => "away",
        }
    }
}

#[component]
pub fn Avatar(
    children: Children,
    #[prop(default = AvatarSize::Md)] size: AvatarSize,
    #[prop(default = AvatarShape::Circle)] shape: AvatarShape,
    #[prop(optional)] status: Option<AvatarStatus>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let status_str = status.map(|s| s.as_str().to_string()).unwrap_or_default();
    view! {
        // wrapper externo — sem overflow:hidden, permite StatusDot pulsar
        <span style="position:relative;display:inline-block;width:fit-content;">
            <AvatarPrimitive
                status=status_str
                size=size.as_str().to_string()
                shape=shape.as_str().to_string()
                class=class
            >
                {children()}
            </AvatarPrimitive>
            {status.map(|s| view! {
                <StatusDotPrimitive
                    variant=s.to_variant()
                    class="avatar-status-dot"
                >
                    ""
                </StatusDotPrimitive>
            })}
        </span>
    }
}

#[component]
pub fn AvatarImage(
    src: String,
    alt: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AvatarImagePrimitive src={src} alt={alt} class={class} />
    }
}

#[component]
pub fn AvatarFallback(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AvatarFallbackPrimitive class={class}>
            {children()}
        </AvatarFallbackPrimitive>
    }
}

#[component]
pub fn AvatarPreview() -> impl IntoView {
    view! { <Avatar size=AvatarSize::Md>"AB"</Avatar> }
}

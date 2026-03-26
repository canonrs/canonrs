use leptos::prelude::*;
use canonrs_core::primitives::{AvatarPrimitive, AvatarImagePrimitive, AvatarFallbackPrimitive};

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
    pub fn as_str(&self) -> &'static str {
        match self { Self::Online=>"online", Self::Offline=>"offline", Self::Busy=>"busy", Self::Away=>"away" }
    }
}

#[component]
pub fn Avatar(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = AvatarSize::Md)] size: AvatarSize,
    #[prop(default = AvatarShape::Circle)] shape: AvatarShape,
    #[prop(optional)] status: Option<AvatarStatus>,
    #[prop(default = false)] animated: bool,
    #[prop(optional)] badge: Option<i32>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AvatarPrimitive
            class={format!("{} avatar-size-{} avatar-shape-{}", class, size.as_str(), shape.as_str())}
            id={id.unwrap_or_default()}
        >
            {children.map(|c| c())}
            {status.map(|s| {
                let pulse = animated && s == AvatarStatus::Online;
                view! {
                    <span
                        data-rs-avatar-status=""
                        data-rs-status={s.as_str()}
                        data-rs-pulse={pulse.then_some("")}
                    />
                }
            })}
            {badge.map(|b| view! {
                <span data-rs-avatar-badge="">{b}</span>
            })}
        </AvatarPrimitive>
    }
}

#[component]
pub fn AvatarImage(
    src: String,
    alt: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
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
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AvatarFallbackPrimitive class={class} id={id.unwrap_or_default()}>
            {children.map(|c| c())}
        </AvatarFallbackPrimitive>
    }
}

#[component]
pub fn AvatarPreview() -> impl IntoView {
    view! { <Avatar size=AvatarSize::Md /> }
}

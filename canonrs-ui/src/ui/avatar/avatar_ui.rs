use leptos::prelude::*;
use crate::primitives::{AvatarPrimitive, AvatarImagePrimitive, AvatarFallbackPrimitive};

#[derive(Clone, Copy, Debug)]
struct AvatarImageError(RwSignal<bool>);

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
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    let img_error = RwSignal::new(false);
    provide_context(AvatarImageError(img_error));

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
                        data-avatar-status=""
                        data-status={s.as_str()}
                        data-pulse={pulse.then_some("")}
                    />
                }
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
    let img_error = expect_context::<AvatarImageError>().0;

    view! {
        <img
            data-avatar-image=""
            src={src}
            alt={alt}
            class={class}
            id={id.unwrap_or_default()}
            class:hidden=move || img_error.get()
            on:error=move |_| img_error.set(true)
        />
    }
}

#[component]
pub fn AvatarFallback(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    let img_error = use_context::<AvatarImageError>().map(|c| c.0);

    view! {
        <span
            data-avatar-fallback=""
            class={class}
            id={id.unwrap_or_default()}
            class:hidden=move || img_error.map_or(false, |sig| !sig.get())
        >
            {children.map(|c| c())}
        </span>
    }
}

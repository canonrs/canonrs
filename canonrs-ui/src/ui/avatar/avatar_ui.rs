use leptos::prelude::*;
use crate::primitives::{AvatarPrimitive, AvatarImagePrimitive, AvatarFallbackPrimitive};

#[derive(Clone, Copy, PartialEq)]
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

#[component]
pub fn Avatar(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = AvatarSize::Md)] size: AvatarSize,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let base_class = format!("avatar size-{} {}", size.as_str(), class);

    view! {
        <AvatarPrimitive
            class={base_class}
            id={id}
        >
            {children.map(|c| c())}
        </AvatarPrimitive>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(default = String::new())] src: String,
    #[prop(default = String::new())] alt: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <AvatarImagePrimitive
            src={src}
            alt={alt}
            class={class}
            id={id}
        />
    }
}

#[component]
pub fn AvatarFallback(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <AvatarFallbackPrimitive
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </AvatarFallbackPrimitive>
    }
}

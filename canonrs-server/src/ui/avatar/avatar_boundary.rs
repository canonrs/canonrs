//! Avatar Boundary — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::avatar_ui::{
    Avatar as AvatarUi,
    AvatarImage as AvatarImageUi,
    AvatarFallback as AvatarFallbackUi,
};
pub use super::avatar_ui::{AvatarSize, AvatarShape, AvatarStatus};

#[component]
pub fn Avatar(
    children: Children,
    #[prop(default = AvatarSize::Md)] size: AvatarSize,
    #[prop(default = AvatarShape::Circle)] shape: AvatarShape,
    #[prop(optional)] status: Option<AvatarStatus>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AvatarUi size=size shape=shape status=status.unwrap_or(AvatarStatus::Offline) class=class>
            {children()}
        </AvatarUi>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AvatarImageUi src=src alt=alt class=class /> }
}

#[component]
pub fn AvatarFallback(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <AvatarFallbackUi class=class>{children()}</AvatarFallbackUi> }
}

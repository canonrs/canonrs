//! @canon-level: strict
//! @canon-owner: primitives-team
//! Container Primitive - Max-width centered layout container

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ContainerSize {
    Sm,
    Md,
    #[default]
    Lg,
    Xl,
    Content,
    Full,
}
impl ContainerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sm      => "sm",
            Self::Md      => "md",
            Self::Lg      => "lg",
            Self::Xl      => "xl",
            Self::Content => "content",
            Self::Full    => "full",
        }
    }
}

#[component]
pub fn ContainerPrimitive(
    children: Children,
    #[prop(default = ContainerSize::Lg)] size: ContainerSize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-container=""
            data-rs-uid=crate::infra::uid::generate("ct")
            data-rs-size=size.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

/// Construtor funcional — retorna AnyView sem passar pelo macro view!
pub fn container_view(
    size:     ContainerSize,
    children: AnyView,
) -> AnyView {
    let uid = crate::infra::uid::generate("ct");
    view! {
        <div
            data-rs-container=""
            data-rs-uid=uid
            data-rs-size=size.as_str()
        >
            {children}
        </div>
    }.into_any()
}

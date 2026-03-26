//! @canon-level: strict
//! @canon-owner: primitives-team
//! Skeleton Primitive - HTML puro

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SkeletonVariant {
    #[default]
    Rectangle,
    Text,
    Circle,
}
impl SkeletonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rectangle => "rectangle",
            Self::Text      => "text",
            Self::Circle    => "circle",
        }
    }
}

#[component]
pub fn SkeletonPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = SkeletonVariant::Rectangle)] variant: SkeletonVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-skeleton=""
            data-rs-component="Skeleton"
            data-rs-behavior="feedback"
            data-rs-variant=variant.as_str()
            aria-busy="true"
            aria-live="polite"
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}

use leptos::prelude::*;
use crate::primitives::SkeletonPrimitive;

#[derive(Clone, Copy, PartialEq)]
pub enum SkeletonVariant {
    Text,
    Circle,
    Rectangle,
}

impl SkeletonVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkeletonVariant::Text => "text",
            SkeletonVariant::Circle => "circle",
            SkeletonVariant::Rectangle => "rectangle",
        }
    }
}

#[component]
pub fn Skeleton(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = SkeletonVariant::Rectangle)] variant: SkeletonVariant,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let base_class = format!("skeleton variant-{} {}", variant.as_str(), class);

    view! {
        <SkeletonPrimitive
            class={base_class}
            id={id}
        >
            {children.map(|c| c())}
        </SkeletonPrimitive>
    }
}

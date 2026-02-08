//! # Skeleton Block
//! Loading placeholder with animation

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SkeletonVariant {
    Text,
    Circle,
    Rectangle,
}

impl SkeletonVariant {
    fn as_str(&self) -> &'static str {
        match self {
            SkeletonVariant::Text => "text",
            SkeletonVariant::Circle => "circle",
            SkeletonVariant::Rectangle => "rectangle",
        }
    }
}

#[component]
pub fn Skeleton(
    #[prop(default = SkeletonVariant::Text)] variant: SkeletonVariant,
    #[prop(optional, into)] width: Option<String>,
    #[prop(optional, into)] height: Option<String>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let style = move || {
        let mut s = String::new();
        if let Some(w) = &width {
            s.push_str(&format!("width: {}; ", w));
        }
        if let Some(h) = &height {
            s.push_str(&format!("height: {}; ", h));
        }
        s
    };

    view! {
        <div 
            class=format!("canon-skeleton canon-skeleton--{} {}", variant.as_str(), class)
            attr:data-block="skeleton"
            style=style
        />
    }
}

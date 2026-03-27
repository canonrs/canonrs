//! @canon-id: carousel-content
//! @canon-label: Carousel Content
//! @canon-family: data_display
//! @canon-category: Display
//! @canon-intent: Carousel content wrapper region
//! @canon-description: Carousel content container
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: carousel, content, region

use leptos::prelude::*;
use super::carousel_content_primitive::CarouselContentPrimitive;

#[component]
pub fn CarouselContent(
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = format!("carousel-content {}", class);

    view! {
        <CarouselContentPrimitive
            class={base_class}
        >
            {children.map(|c| c())}
        </CarouselContentPrimitive>
    }
}

//! @canon-id: carousel-item
//! @canon-label: Carousel Item
//! @canon-family: data_display
//! @canon-category: Display
//! @canon-intent: Single item inside a carousel track
//! @canon-description: Individual carousel slide item
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: carousel, item, slide

use leptos::prelude::*;
use super::carousel_item_primitive::CarouselItemPrimitive;

#[component]
pub fn CarouselItem(
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = format!("carousel-item {}", class);

    view! {
        <CarouselItemPrimitive
            class={base_class}
        >
            {children.map(|c| c())}
        </CarouselItemPrimitive>
    }
}

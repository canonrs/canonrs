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

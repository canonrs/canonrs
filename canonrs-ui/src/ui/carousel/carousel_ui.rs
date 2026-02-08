use leptos::prelude::*;
use super::carousel_primitive::CarouselPrimitive;

#[component]
pub fn Carousel(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = format!("carousel {}", class);

    view! {
        <CarouselPrimitive
            id={id}
            class={base_class}
        >
            {children.map(|c| c())}
        </CarouselPrimitive>
    }
}

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

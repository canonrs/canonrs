use leptos::prelude::*;

#[component]
pub fn CarouselItemPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            class={class}
            attr:data-carousel-item=""
        >
            {children.map(|c| c())}
        </div>
    }
}

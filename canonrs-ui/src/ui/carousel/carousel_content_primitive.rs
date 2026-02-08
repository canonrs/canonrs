use leptos::prelude::*;

#[component]
pub fn CarouselContentPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            class={class}
            attr:data-carousel-content=""
        >
            {children.map(|c| c())}
        </div>
    }
}

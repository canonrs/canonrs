//! @canon-level: strict
//! CarouselContent Primitive
use leptos::prelude::*;
use crate::infra::uid::generate;

#[component]
pub fn CarouselContentPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let uid = generate("cc");
    view! {
        <div
            data-rs-carousel-content=""
            data-rs-uid=uid
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}

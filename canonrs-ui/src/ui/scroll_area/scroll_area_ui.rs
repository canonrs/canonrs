use leptos::prelude::*;
use super::scroll_area_primitive::ScrollAreaPrimitive;

#[component]
pub fn ScrollArea(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = format!("scroll-area {}", class);

    view! {
        <ScrollAreaPrimitive
            id={id}
            class={base_class}
        >
            {children.map(|c| c())}
        </ScrollAreaPrimitive>
    }
}

use leptos::prelude::*;

#[component]
pub fn ResizablePrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] direction: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            id={id}
            class={class}
            attr:data-resizable=""
            attr:data-direction={direction}
        >
            {children.map(|c| c())}
        </div>
    }
}

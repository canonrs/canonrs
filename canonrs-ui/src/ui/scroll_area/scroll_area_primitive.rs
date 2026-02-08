use leptos::prelude::*;

#[component]
pub fn ScrollAreaPrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            id={id}
            class={class}
            attr:data-scroll-area=""
        >
            {children.map(|c| c())}
        </div>
    }
}

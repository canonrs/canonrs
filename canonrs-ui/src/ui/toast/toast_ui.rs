use leptos::prelude::*;
use crate::primitives::ToastClosePrimitive;

#[component]
pub fn ToastClose(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ToastClosePrimitive class={class} id={id}>
            {children.map(|c| c())}
        </ToastClosePrimitive>
    }
}

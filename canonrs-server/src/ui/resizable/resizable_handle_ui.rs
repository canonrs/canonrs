#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::ResizableHandlePrimitive;

#[component]
pub fn ResizableHandle(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! { <ResizableHandlePrimitive class=class id=id disabled=disabled /> }
}

use leptos::prelude::*;
use super::resizable_handle_primitive::ResizableHandlePrimitive;

#[component]
pub fn ResizableHandle(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let base_class = format!("resizable-handle {}", class);

    view! {
        <ResizableHandlePrimitive
            id={id}
            class={base_class}
            disabled={disabled}
        />
    }
}

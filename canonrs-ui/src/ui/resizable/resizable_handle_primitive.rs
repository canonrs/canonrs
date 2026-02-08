use leptos::prelude::*;

#[component]
pub fn ResizableHandlePrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <div
            id={id}
            class={class}
            attr:data-resizable-handle=""
            attr:aria-disabled={if disabled { "true" } else { "false" }}
        />
    }
}

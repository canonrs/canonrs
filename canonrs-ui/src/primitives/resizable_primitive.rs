use leptos::prelude::*;

#[component]
pub fn ResizablePrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div id={id} class={class} data-resizable="">
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ResizablePanelPrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div id={id} class={class}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ResizableHandlePrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div id={id} class={class} data-resizable-handle="" aria-disabled="false"></div>
    }
}

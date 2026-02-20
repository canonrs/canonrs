//! @canon-level: strict
//! Resizable Primitives - Split panels with draggable divider

use leptos::prelude::*;

#[component]
pub fn ResizablePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-resizable=""
            role="group"
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ResizablePanelPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-resizable-panel=""
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
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
        <div
            data-resizable-handle=""
            id={id}
            role="separator"
            tabindex="0"
            class={class}
        >
            <div data-resizable-handle-bar=""></div>
        </div>
    }
}

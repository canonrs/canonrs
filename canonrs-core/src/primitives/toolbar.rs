//! @canon-level: strict
//! Toolbar Primitives - Application toolbar container

use leptos::prelude::*;

#[component]
pub fn ToolbarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] aria_label: String,
    #[prop(default = "horizontal".to_string())] orientation: String,
) -> impl IntoView {
    view! {
        <div
            data-toolbar=""
            role="toolbar"
            aria-label={aria_label}
            aria-orientation={orientation}
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ToolbarSeparatorPrimitive(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-toolbar-separator=""
            role="separator"
            aria-orientation="vertical"
            class={class}
        />
    }
}

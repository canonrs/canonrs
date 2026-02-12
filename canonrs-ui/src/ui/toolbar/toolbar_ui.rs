use leptos::prelude::*;
use crate::primitives::ToolbarPrimitive;

#[component]
pub fn Toolbar(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ToolbarPrimitive
            class={class}
            id={id.unwrap_or_default()}
        >
            {children.map(|c| c())}
        </ToolbarPrimitive>
    }
}

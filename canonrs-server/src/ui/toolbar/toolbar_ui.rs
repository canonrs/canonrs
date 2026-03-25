use leptos::prelude::*;
use canonrs_core::primitives::{ToolbarPrimitive, ToolbarSeparatorPrimitive, ToolbarOrientation};

#[component]
pub fn Toolbar(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] aria_label: String,
    #[prop(default = ToolbarOrientation::Horizontal)] orientation: ToolbarOrientation,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ToolbarPrimitive
            class=class
            id={id.unwrap_or_default()}
            aria_label=aria_label
            orientation=orientation
        >
            {children.map(|c| c())}
        </ToolbarPrimitive>
    }
}

#[component]
pub fn ToolbarSeparator(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ToolbarSeparatorPrimitive class=class />
    }
}

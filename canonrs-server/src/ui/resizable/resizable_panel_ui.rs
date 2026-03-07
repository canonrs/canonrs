use leptos::prelude::*;
use super::resizable_panel_primitive::ResizablePanelPrimitive;

#[component]
pub fn ResizablePanel(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let base_class = format!("resizable-panel {}", class);

    view! {
        <ResizablePanelPrimitive
            id={id}
            class={base_class}
        >
            {children.map(|c| c())}
        </ResizablePanelPrimitive>
    }
}

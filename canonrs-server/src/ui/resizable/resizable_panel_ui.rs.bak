//! @canon-id: resizable-panel
//! @canon-label: Resizable Panel
//! @canon-family: layout
//! @canon-category: Layout
//! @canon-intent: Panel region inside a resizable container
//! @canon-description: Resizable panel region
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: resizable-panel, resizable, panel, region, split

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

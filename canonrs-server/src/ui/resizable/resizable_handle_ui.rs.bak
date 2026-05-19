//! @canon-id: resizable-handle
//! @canon-label: Resizable Handle
//! @canon-family: layout
//! @canon-category: Layout
//! @canon-intent: Drag handle between resizable panels
//! @canon-description: Draggable resize handle
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: resizable-handle, resizable, handle, drag, divider

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

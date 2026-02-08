use leptos::prelude::*;
use super::drag_context::use_drag_context;
use super::types::DragState;
use std::sync::Arc;

#[component]
pub fn DragOverlay<F, V>(
    render: F,
) -> impl IntoView
where
    F: Fn(String, Option<String>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let context = use_drag_context();
    let render = Arc::new(render);

    view! {
        <div
            attr:data-drag-overlay=""
            attr:data-active={move || matches!(context.state.get(), DragState::Dragging { .. })}
        >
            {move || {
                let render = Arc::clone(&render);
                match context.state.get() {
                    DragState::Dragging { item_id, data, .. } => {
                        Some(render(item_id.0.clone(), data))
                    }
                    DragState::Idle => None,
                }
            }}
        </div>
    }
}

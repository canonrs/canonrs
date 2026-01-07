use leptos::prelude::*;
use super::drag_context::use_drag_context;

/// DragOverlay - Visual preview that follows cursor during drag
#[component]
pub fn DragOverlay<F, IV>(
    render: F,
) -> impl IntoView
where
    F: Fn(String, Option<String>) -> IV + Send + Sync + 'static,
    IV: IntoView,
{
    #[cfg(target_arch = "wasm32")]
    {
        let context = use_drag_context();
        
        view! {
            {move || {
                let state = context.state.get();
                
                if let super::types::DragState::Dragging { item_id, data, .. } = state {
                    view! {
                        <div
                            class="drag-overlay fixed pointer-events-none z-50 opacity-80"
                            style="left: 50%; top: 50%; transform: translate(-50%, -50%);"
                        >
                            {render(item_id.0.clone(), data)}
                        </div>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        view! { <></> }
    }
}

use leptos::prelude::*;
use super::types::DragItemId;
use super::drag_context::use_drag_context;

/// DragHandle - Makes an element draggable
///
/// **Type:** Pure Component (Type 1)
/// **Client-only:** Yes (uses DOM events)
#[component]
pub fn DragHandle(
    /// Unique ID for this draggable item
    #[prop(into)]
    item_id: DragItemId,
    
    /// Optional data payload to carry during drag
    #[prop(optional)]
    data: Option<String>,
    
    /// Children (visual content)
    children: Children,
    
    /// Additional CSS classes
    #[prop(default = String::new(), into)]
    class: String,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let context = use_drag_context();
        
        let on_drag_start = move |ev: web_sys::DragEvent| {
            
            leptos::logging::log!("🟢 DragHandle: dragstart - item_id={:?}", item_id);
            context.start_drag(item_id.clone(), data.clone());
            
            // Set drag image (optional - can customize later)
            if let Some(dt) = ev.data_transfer() {
                let _ = dt.set_effect_allowed("move");
            }
        };
        
        let on_drag_end = move |_ev: web_sys::DragEvent| {
            // context.cancel_drag(); // Bug: dragend fires BEFORE drop
        };
        
        view! {
            <div
                class=format!("drag-handle cursor-grab active:cursor-grabbing {}", class)
                draggable="true"
                on:dragstart=on_drag_start
                on:dragend=on_drag_end
            >
                {children()}
            </div>
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // SSR: render without drag functionality
        view! {
            <div class=format!("drag-handle {}", class)>
                {children()}
            </div>
        }
    }
}

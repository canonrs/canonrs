use leptos::prelude::*;
use super::types::DropTargetId;
use super::drag_context::use_drag_context;

/// DropZone - Area that accepts dropped items
///
/// **Type:** Pure Component (Type 1)
/// **Client-only:** Yes (uses DOM events)
#[component]
pub fn DropZone(
    /// Unique ID for this drop target
    #[prop(into)]
    target_id: DropTargetId,
    
    /// Children (visual content)
    children: Children,
    
    /// Additional CSS classes
    #[prop(default = String::new(), into)]
    class: String,
    
    /// CSS class when item is hovering
    #[prop(default = "bg-primary/10 border-primary".to_string(), into)]
    hover_class: String,
) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let context = use_drag_context();
        let target_id_clone = target_id.clone();
        
        let is_hovering = Signal::derive(move || {
            context.state.get().is_hovering(&target_id_clone)
        });
        
        let target_for_over = target_id.clone();
        let on_drag_over = move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            context.set_hover(Some(target_for_over.clone()));
            
            if let Some(dt) = ev.data_transfer() {
                dt.set_drop_effect("move");
            }
        };
        
        let on_drag_leave = move |_ev: web_sys::DragEvent| {
            context.set_hover(None);
        };
        
        let target_for_drop = target_id.clone();
        let on_drop = move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            leptos::logging::log!("🔵 DropZone: drop - target_id={:?}", target_for_drop);
            context.drop_item(target_for_drop.clone());
        };
        
        view! {
            <div
                class=move || {
                    let base = format!("drop-zone {}", class);
                    if is_hovering.get() {
                        format!("{} {}", base, hover_class)
                    } else {
                        base
                    }
                }
                on:dragover=on_drag_over
                on:dragleave=on_drag_leave
                on:drop=on_drop
            >
                {children()}
            </div>
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // SSR: render without drop functionality
        view! {
            <div class=format!("drop-zone {}", class)>
                {children()}
            </div>
        }
    }
}

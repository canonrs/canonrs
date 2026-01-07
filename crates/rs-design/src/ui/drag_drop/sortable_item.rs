use leptos::prelude::*;
use super::types::DragItemId;
use super::drag_context::use_drag_context;
use super::drag_handle::DragHandle;
use super::drop_indicator::DropIndicator;

/// SortableItem - Item with drop zones above and below
///
/// **Type:** Stateful Component (Type 2)
#[component]
pub fn SortableItem(
    /// Unique ID for this item
    #[prop(into)]
    item_id: DragItemId,
    
    /// Index in list
    index: usize,
    
    /// Callback when dropped before this item
    #[prop(into)]
    on_drop_before: Callback<(DragItemId, usize)>,
    
    /// Callback when dropped after this item
    #[prop(into)]
    on_drop_after: Callback<(DragItemId, usize)>,
    
    /// Children (item content)
    children: Children,
) -> impl IntoView {
    let context = use_drag_context();
    
    let (hover_before, set_hover_before) = signal(false);
    let (hover_after, set_hover_after) = signal(false);
    
    let item_id_clone = item_id.clone();
    let is_dragging = Signal::derive(move || {
        context.state.get().dragging_item() == Some(&item_id_clone)
    });
    
    #[cfg(target_arch = "wasm32")]
    {
        let on_drag_over_before = move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            set_hover_before.set(true);
        };
        
        let on_drag_leave_before = move |_ev: web_sys::DragEvent| {
            set_hover_before.set(false);
        };
        
        let on_drop_before_handler = move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            if let Some(dragging_id) = context.state.get().dragging_item() {
                on_drop_before.run((dragging_id.clone(), index));
            }
            set_hover_before.set(false);
            context.cancel_drag();
        };
        
        let on_drag_over_after = move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            set_hover_after.set(true);
        };
        
        let on_drag_leave_after = move |_ev: web_sys::DragEvent| {
            set_hover_after.set(false);
        };
        
        let on_drop_after_handler = move |ev: web_sys::DragEvent| {
            ev.prevent_default();
            if let Some(dragging_id) = context.state.get().dragging_item() {
                on_drop_after.run((dragging_id.clone(), index));
            }
            set_hover_after.set(false);
            context.cancel_drag();
        };
        
        view! {
            <div class="relative" style=move || if is_dragging.get() { "opacity: 0.5" } else { "" }>
                <div
                    class="absolute top-0 left-0 right-0 h-2 -mt-1 z-10"
                    on:dragover=on_drag_over_before
                    on:dragleave=on_drag_leave_before
                    on:drop=on_drop_before_handler
                >
                    <DropIndicator visible=hover_before position="top".to_string() />
                </div>
                
                <DragHandle item_id=item_id>
                    {children()}
                </DragHandle>
                
                <div
                    class="absolute bottom-0 left-0 right-0 h-2 -mb-1 z-10"
                    on:dragover=on_drag_over_after
                    on:dragleave=on_drag_leave_after
                    on:drop=on_drop_after_handler
                >
                    <DropIndicator visible=hover_after position="bottom".to_string() />
                </div>
            </div>
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // SSR: mesma estrutura mas sem eventos
        view! {
            <div class="relative">
                <div class="absolute top-0 left-0 right-0 h-2 -mt-1 z-10">
                    <DropIndicator visible=hover_before position="top".to_string() />
                </div>
                
                <DragHandle item_id=item_id>
                    {children()}
                </DragHandle>
                
                <div class="absolute bottom-0 left-0 right-0 h-2 -mb-1 z-10">
                    <DropIndicator visible=hover_after position="bottom".to_string() />
                </div>
            </div>
        }
    }
}

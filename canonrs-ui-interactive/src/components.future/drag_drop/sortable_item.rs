use leptos::callback::Callback;
use leptos::prelude::*;
use super::types::{DragItemId, DropTargetId};
use super::drag_context::use_drag_context;
use super::drag_handle::DragHandle;
use super::drop_indicator::DropIndicator;
use super::drop_zone::DropZone;

#[component]
pub fn SortableItem(
    item_id: DragItemId,
    _index: usize,
    _on_drop_before: Callback<(DragItemId, usize)>,
    _on_drop_after: Callback<(DragItemId, usize)>,
    children: Children,
) -> impl IntoView {
    let context = use_drag_context();
    let item_id_clone = item_id.clone();
    
    let is_dragging = Memo::new(move |_| {
        context.state.get().dragging_item() == Some(&item_id_clone)
    });
    
    let target_before = DropTargetId::new(format!("before-{}", item_id.0));
    let target_after = DropTargetId::new(format!("after-{}", item_id.0));
    
    let target_before_clone = target_before.clone();
    let target_after_clone = target_after.clone();
    
    let visible_before = RwSignal::new(false);
    let visible_after = RwSignal::new(false);
    
    Effect::new(move |_| {
        visible_before.set(context.state.get().is_hovering(&target_before_clone));
    });
    
    Effect::new(move |_| {
        visible_after.set(context.state.get().is_hovering(&target_after_clone));
    });
    
    view! {
        <div
            attr:data-sortable-item=""
            attr:data-dragging={is_dragging.get()}
        >
            <DropZone
                target_id={target_before}
            >
                <DropIndicator
                    visible={visible_before.into()}
                    position="top".to_string()
                />
            </DropZone>
            
            <DragHandle item_id={item_id.clone()}>
                {children()}
            </DragHandle>
            
            <DropZone
                target_id={target_after}
            >
                <DropIndicator
                    visible={visible_after.into()}
                    position="bottom".to_string()
                />
            </DropZone>
        </div>
    }
}

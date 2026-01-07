use leptos::prelude::*;
use super::types::DragItemId;
use super::sortable_item::SortableItem;

/// SortableList - Reorderable list with visual drop indicators
///
/// **Type:** Stateful Component (Type 2)
#[component]
pub fn SortableList<T, F, R, IV>(
    /// Items to render
    #[prop(into)]
    items: Signal<Vec<T>>,
    
    /// Callback when items are reordered
    #[prop(into)]
    on_reorder: Callback<Vec<T>>,
    
    /// Get ID from item
    item_id: F,
    
    /// Render function for each item: (item: T) -> View
    render: R,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&T) -> String + Copy + Send + Sync + 'static,
    R: Fn(T) -> IV + Copy + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let handle_drop = move |(drag_id, drop_index): (DragItemId, usize)| {
        let current_items = items.get();
        
        // Find dragged item
        let drag_item_idx = current_items
            .iter()
            .position(|item| item_id(item) == drag_id.0);
        
        if let Some(from_idx) = drag_item_idx {
            let mut new_items = current_items.clone();
            let item = new_items.remove(from_idx);
            
            // Adjust target index if needed
            let to_idx = if from_idx < drop_index {
                drop_index - 1
            } else {
                drop_index
            };
            
            new_items.insert(to_idx, item);
            on_reorder.run(new_items);
        }
    };
    
    view! {
        <div class="space-y-2">
            {move || {
                items.get()
                    .into_iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        let id = item_id(&item);
                        
                        view! {
                            <SortableItem
                                item_id=DragItemId::new(id)
                                index=idx
                                on_drop_before=Callback::new(move |(drag_id, _)| {
                                    handle_drop((drag_id, idx))
                                })
                                on_drop_after=Callback::new(move |(drag_id, _)| {
                                    handle_drop((drag_id, idx + 1))
                                })
                            >
                                {render(item)}
                            </SortableItem>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

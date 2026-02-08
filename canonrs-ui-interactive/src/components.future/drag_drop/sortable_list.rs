use leptos::callback::Callback;
use leptos::prelude::*;
use super::types::DragItemId;
use super::sortable_item::SortableItem;

#[component]
pub fn SortableList<T, F, R, V>(
    items: RwSignal<Vec<T>>,
    on_reorder: Callback<Vec<T>>,
    item_id: F,
    render: R,
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(&T) -> String + Copy + Send + Sync + 'static,
    R: Fn(T) -> V + Copy + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let handle_drop = Callback::new(move |(drag_id, drop_index): (DragItemId, usize)| {
        let current = items.get_untracked();
        let drag_item_idx = current
            .iter()
            .position(|item| item_id(item) == drag_id.0);

        if let Some(from_idx) = drag_item_idx {
            let mut new_items = current.clone();
            let item = new_items.remove(from_idx);
            let to_idx = if from_idx < drop_index { drop_index - 1 } else { drop_index };
            new_items.insert(to_idx, item);
            items.set(new_items.clone());
            on_reorder.run(new_items);
        }
    });

    view! {
        <div data-sortable-list="">
            {move || {
                items.get().iter().enumerate().map(|(idx, item)| {
                    let item_key = item_id(item);
                    let item_id_val = DragItemId(item_key.clone());
                    let item_clone = item.clone();
                    let handle_drop_clone = handle_drop.clone();
                    view! {
                        <SortableItem
                            item_id=item_id_val
                            _index=idx
                            _on_drop_before=handle_drop_clone
                            _on_drop_after=handle_drop_clone
                        >
                            {render(item_clone)}
                        </SortableItem>
                    }
                }).collect_view()
            }}
        </div>
    }
}

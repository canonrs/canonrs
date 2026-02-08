use leptos::callback::Callback;
use leptos::prelude::*;
use super::types::{DragState, DragItemId, DropTargetId, DropEvent};

#[derive(Clone, Copy)]
pub struct DragContext {
    pub state: RwSignal<DragState>,
    on_drop_trigger: RwSignal<Option<Callback<DropEvent>>>,
}

impl DragContext {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(DragState::Idle),
            on_drop_trigger: RwSignal::new(None),
        }
    }

    pub fn set_drop_handler(&self, handler: Callback<DropEvent>) {
        self.on_drop_trigger.set(Some(handler));
    }

    pub fn start_drag(&self, item_id: DragItemId, data: Option<String>) {
        self.state.set(DragState::Dragging {
            item_id,
            data,
            hover_target: None,
        });
    }

    pub fn set_hover(&self, target_id: Option<DropTargetId>) {
        self.state.update(|state: &mut DragState| {
            if let DragState::Dragging { item_id, data, .. } = state {
                *state = DragState::Dragging {
                    item_id: item_id.clone(),
                    data: data.clone(),
                    hover_target: target_id,
                };
            }
        });
    }

    pub fn drop_item(&self, target_id: DropTargetId) {
        let current_state = self.state.get();

        if let DragState::Dragging { item_id, data, .. } = current_state {
            if let Some(handler) = self.on_drop_trigger.get() {
                handler.run(DropEvent {
                    item_id,
                    target_id,
                    data,
                });
            }
        }

        self.state.set(DragState::Idle);
    }

    pub fn cancel_drag(&self) {
        self.state.set(DragState::Idle);
    }
}

pub fn use_drag_context() -> DragContext {
    use_context::<DragContext>()
        .expect("DragContext must be provided")
}

#[component]
pub fn DragProvider(children: Children) -> impl IntoView {
    let context = DragContext::new();
    provide_context(context);

    view! { {children()} }
}

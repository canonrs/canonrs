use leptos::prelude::*;
use super::types::{DragState, DragItemId, DropTargetId, DropEvent};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct DragContext {
    pub state: RwSignal<DragState>,
    pub on_drop_trigger: RwSignal<Option<Arc<dyn Fn(DropEvent) + Send + Sync>>>,
}

impl DragContext {
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(DragState::Idle),
            on_drop_trigger: RwSignal::new(None),
        }
    }
    
    pub fn set_drop_handler<F>(&self, handler: F)
    where
        F: Fn(DropEvent) + Send + Sync + 'static,
    {
        self.on_drop_trigger.set(Some(Arc::new(handler)));
        leptos::logging::log!("✅ Drop handler registered in DragContext");
    }

    pub fn start_drag(&self, item_id: DragItemId, data: Option<String>) {
        leptos::logging::log!("🟢 DragContext: start_drag - item={:?}", item_id);
        self.state.set(DragState::Dragging {
            item_id,
            data,
            hover_target: None,
        });
    }

    pub fn set_hover(&self, target_id: Option<DropTargetId>) {
        self.state.update(|state| {
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
        leptos::logging::log!("🟡 DragContext: drop_item - target={:?}", target_id);
        let current_state = self.state.get();
        leptos::logging::log!("🔎 Current state: {:?}", current_state);
        
        if let DragState::Dragging { item_id, data, .. } = current_state {
        let handler_exists = self.on_drop_trigger.get().is_some();
        leptos::logging::log!("🔍 Handler exists in drop_item: {}", handler_exists);
            if let Some(handler) = self.on_drop_trigger.get() {
                leptos::logging::log!("✅ Drop handler found - executing");
                handler(DropEvent {
                    item_id,
                    target_id,
                    data,
                });
            } else {
                leptos::logging::log!("❌ No drop handler registered!");
            }
        }
        
        self.state.set(DragState::Idle);
    }

    pub fn cancel_drag(&self) {
        self.state.set(DragState::Idle);
    }
}

#[component]
pub fn DragDropProvider(children: Children) -> impl IntoView {
    let context = DragContext::new();
    leptos::logging::log!("🟦 DragDropProvider mounting - context created");
    provide_context(context);
    children()
}

pub fn use_drag_context() -> DragContext {
    expect_context::<DragContext>()
}

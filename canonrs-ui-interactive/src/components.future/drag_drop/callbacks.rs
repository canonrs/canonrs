use leptos::prelude::*;
use super::types::DropEvent;
use super::drag_context::use_drag_context;

#[derive(Clone, Copy)]
pub struct DragDropCallbacks {
    pub on_drop: RwSignal<Vec<Callback<DropEvent>>>,
}

impl DragDropCallbacks {
    pub fn new() -> Self {
        Self {
            on_drop: RwSignal::new(Vec::new()),
        }
    }

    pub fn register_drop(&self, callback: Callback<DropEvent>) {
        self.on_drop.update(|callbacks| {
            callbacks.push(callback);
        });
    }

    pub fn trigger_drop(&self, event: DropEvent) {
        let callbacks = self.on_drop.get();
        for callback in callbacks.iter() {
            callback.run(event.clone());
        }
    }
}

#[component]
pub fn DragDropCallbacksProvider(children: Children) -> impl IntoView {
    let callbacks = DragDropCallbacks::new();
    provide_context(callbacks);

    let drag_ctx = use_drag_context();

    Effect::new(move |_| {
        drag_ctx.set_drop_handler(Callback::new(move |event: DropEvent| {
            callbacks.trigger_drop(event);
        }));
    });

    view! { {children()} }
}

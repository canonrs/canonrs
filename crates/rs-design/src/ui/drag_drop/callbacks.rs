use leptos::prelude::*;
use super::types::DropEvent;
use super::drag_context::use_drag_context;

#[derive(Clone, Copy)]
pub struct DragDropCallbacks {
    pub on_drop: RwSignal<Vec<Callback<DropEvent>>>,
}

impl DragDropCallbacks {
    pub fn new() -> Self {
        leptos::logging::log!("🏗️ DragDropCallbacks::new() called");
        Self {
            on_drop: RwSignal::new(Vec::new()),
        }
    }
    
    pub fn register_drop(&self, callback: Callback<DropEvent>) {
        leptos::logging::log!("📝 Registering drop callback");
        self.on_drop.update(|cbs| {
            cbs.push(callback);
            leptos::logging::log!("📝 Total callbacks: {}", cbs.len());
        });
    }
    
    pub fn trigger_drop(&self, event: DropEvent) {
        let callbacks = self.on_drop.get_untracked();
        leptos::logging::log!("🎯 Triggering {} callbacks for event: {:?}", callbacks.len(), event);
        for (i, callback) in callbacks.iter().enumerate() {
            leptos::logging::log!("🎯 Calling callback #{}", i);
            callback.run(event.clone());
        }
    }
}

#[component]
pub fn DragDropCallbacksProvider(children: Children) -> impl IntoView {
    leptos::logging::log!("🔧 DragDropCallbacksProvider component mounting...");
    let callbacks = DragDropCallbacks::new();
    provide_context(callbacks);
    
    leptos::logging::log!("🔗 Getting DragContext...");
    let drag_ctx = use_drag_context();
    
    leptos::logging::log!("🔗 Setting drop handler...");
    let cbs = callbacks;
    drag_ctx.set_drop_handler(move |event: DropEvent| {
        leptos::logging::log!("🎬 Drop handler closure called!");
        cbs.trigger_drop(event);
    });
    leptos::logging::log!("✅ Drop handler connected to DragContext");
    
    children()
}

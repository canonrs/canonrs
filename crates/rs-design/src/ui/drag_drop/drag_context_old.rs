use leptos::prelude::*;
use super::types::{DragState, DragStartEvent, DropEvent, DragItemId, DropTargetId};
use std::sync::atomic::{AtomicU32, Ordering};

static CONTEXT_COUNTER: AtomicU32 = AtomicU32::new(0);

/// DragContext - Global drag & drop state
#[derive(Clone, Copy)]
pub struct DragContext {
    context_id: u32,
    
    /// Current drag state
    pub state: RwSignal<DragState>,
    
    /// Currently hovered drop target
    hover: RwSignal<Option<DropTargetId>>,
    
    /// Callback when drag starts
    pub on_drag_start: StoredValue<Option<Callback<DragStartEvent>>>,
    
    /// Callback when item is dropped
    pub on_drop: StoredValue<Option<Callback<DropEvent>>>,
}

impl DragContext {
    pub fn new() -> Self {
        let id = 0;
        Self {
            context_id: id,
            state: RwSignal::new(DragState::Idle),
            hover: RwSignal::new(None),
            on_drag_start: StoredValue::new(None),
            on_drop: StoredValue::new(None),
        }
    }
    
    /// Start dragging an item
    pub fn start_drag(&self, item_id: DragItemId, data: String) {
        leptos::logging::log!("🆔 Context ID: {}", self.context_id);
        leptos::logging::log!("🎯 start_drag: item_id={}", item_id.0);
        self.state.set(DragState::Dragging {
            item_id: item_id.clone(),
            data: Some(data.clone()),
            hover_target: None,
        });
        leptos::logging::log!("✅ State set to Dragging");
        leptos::logging::log!("🔍 State after set: {:?}", self.state.get());
        
        // Emit drag start event
        if let Some(callback) = self.on_drag_start.get_value() {
            callback.run(DragStartEvent { 
                item_id, 
                data: Some(data),
            });
        }
    }
    
    /// Set hover target
    pub fn set_hover(&self, target: Option<DropTargetId>) {
        self.hover.set(target);
    }
    
    /// Get current hover target
    pub fn is_hovering(&self, target: &DropTargetId) -> Signal<bool> {
        let hover = self.hover;
        let target = target.clone();
        Signal::derive(move || {
            hover.get().as_ref() == Some(&target)
        })
    }
    
    /// Drop item on target
    pub fn drop_item(&self, target_id: DropTargetId) {
        leptos::logging::log!("🆔 Context ID: {}", self.context_id);
        leptos::logging::log!("🎯 drop_item called: target={}", target_id.0);
        leptos::logging::log!("📍 About to get state...");
        let current_state = self.state.get();
        leptos::logging::log!("📍 Got state successfully");
        leptos::logging::log!("🔍 Current drag state: {:?}", current_state);

        if let DragState::Dragging { item_id, data, .. } = current_state {
            // Emit drop event
            if let Some(callback) = self.on_drop.get_value() {
                leptos::logging::log!("✅ Callback exists, emitting event");
                callback.run(DropEvent {
                    item_id: item_id.clone(),
                    target_id,
                    data,
                });
            } else {
                leptos::logging::log!("❌ No callback registered!");
            }
            
            // Clear drag state
            self.state.set(DragState::Idle);
        } else {
            leptos::logging::log!("❌ Not in dragging state!");
        }
    }
    
    /// Cancel drag (alias for end_drag)
    pub fn cancel_drag(&self) {
        self.end_drag();
    }
    
    /// End drag (without drop)
    pub fn end_drag(&self) {
        self.state.set(DragState::Idle);
        self.hover.set(None);
    }
}

/// DragDropProvider - Provides drag & drop context
#[component]
pub fn DragDropProvider(
    /// Callback when drag starts
    #[prop(optional, into)]
    on_drag_start: Option<Callback<DragStartEvent>>,
    
    /// Callback when item is dropped
    #[prop(optional, into)]
    on_drop: Option<Callback<DropEvent>>,
    
    /// Children
    children: Children,
) -> impl IntoView {
    let context = DragContext::new();
    
    context.on_drag_start.set_value(on_drag_start);
    context.on_drop.set_value(on_drop);
    
    provide_context(context);
    
    children()
}

/// Hook to access drag context
/// Hook to access drag context (with fallback)
/// Hook to access drag context (with fallback)
pub fn use_drag_context() -> DragContext {
    match leptos::prelude::use_context::<DragContext>() {
        Some(ctx) => {
            leptos::logging::log!("✅ Found context ID: {}", ctx.context_id);
            ctx
        },
        None => {
            leptos::logging::log!("⚠️ No DragContext found, creating temporary one");
            DragContext::new()
        }
    }
}

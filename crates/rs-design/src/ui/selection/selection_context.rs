use leptos::prelude::*;
use super::types::{SelectionMode, SelectionState};

/// SelectionContext - Manages multi-selection state
#[derive(Clone, Copy)]
pub struct SelectionContext<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static,
{
    /// Current selection state
    pub state: RwSignal<SelectionState<T>>,
    
    /// Selection mode
    pub mode: SelectionMode,
    
    /// Callback when selection changes
    pub on_change: StoredValue<Option<Callback<SelectionState<T>>>>,
}

impl<T> SelectionContext<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static,
{
    pub fn new(mode: SelectionMode) -> Self {
        Self {
            state: RwSignal::new(SelectionState::Empty),
            mode,
            on_change: StoredValue::new(None),
        }
    }
    
    /// Select single item (clears previous selection)
    pub fn select_single(&self, item: T) {
        if self.mode == SelectionMode::None {
            return;
        }
        
        self.state.update(|s| s.set_single(item));
        self.emit_change();
    }
    
    /// Toggle item selection
    pub fn toggle(&self, item: T) {
        if self.mode == SelectionMode::None {
            return;
        }
        
        if self.mode == SelectionMode::Single {
            self.select_single(item);
        } else {
            self.state.update(|s| s.toggle(item));
            self.emit_change();
        }
    }
    
    /// Add item to selection (multi-select)
    pub fn add(&self, item: T) {
        if self.mode != SelectionMode::Multiple {
            return;
        }
        
        self.state.update(|s| s.add(item));
        self.emit_change();
    }
    
    /// Remove item from selection
    pub fn remove(&self, item: &T) {
        self.state.update(|s| s.remove(item));
        self.emit_change();
    }
    
    /// Clear all selections
    pub fn clear(&self) {
        self.state.update(|s| s.clear());
        self.emit_change();
    }
    
    /// Check if item is selected
    pub fn is_selected(&self, item: &T) -> Signal<bool> {
        let state = self.state;
        let item = item.clone();
        Signal::derive(move || state.get().is_selected(&item))
    }
    
    /// Get selection count
    pub fn count(&self) -> Signal<usize> {
        let state = self.state;
        Signal::derive(move || state.get().count())
    }
    
    /// Get all selected items
    pub fn get_all(&self) -> Vec<T> {
        self.state.get().get_all()
    }
    
    fn emit_change(&self) {
        if let Some(callback) = self.on_change.get_value() {
            callback.run(self.state.get());
        }
    }
}

/// SelectionProvider - Provides selection context
#[component]
pub fn SelectionProvider<T>(
    /// Selection mode
    #[prop(default = SelectionMode::Multiple)]
    mode: SelectionMode,
    
    /// Callback when selection changes
    #[prop(optional, into)]
    on_change: Option<Callback<SelectionState<T>>>,
    
    /// Children
    children: Children,
) -> impl IntoView
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static,
{
    let context = SelectionContext::new(mode);
    context.on_change.set_value(on_change);
    
    provide_context(context);
    
    children()
}

/// Hook to access selection context
pub fn use_selection<T>() -> SelectionContext<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static,
{
    expect_context::<SelectionContext<T>>()
}

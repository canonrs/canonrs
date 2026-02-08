use leptos::callback::Callback;
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

    /// Callback when selection changes (internal)
    on_change: RwSignal<Option<Callback<SelectionState<T>>>>,
}

impl<T> SelectionContext<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static,
{
    pub fn new(mode: SelectionMode) -> Self {
        Self {
            state: RwSignal::new(SelectionState::Empty),
            mode,
            on_change: RwSignal::new(None),
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

        self.state.update(|s| {
            if self.mode == SelectionMode::Single {
                if s.is_selected(&item) {
                    s.clear();
                } else {
                    s.set_single(item);
                }
            } else {
                s.toggle(item);
            }
        });
        self.emit_change();
    }

    /// Select multiple items
    pub fn select_multiple(&self, items: Vec<T>) {
        if self.mode != SelectionMode::Multiple {
            return;
        }

        self.state.update(|s| {
            for item in items {
                s.add(item);
            }
        });
        self.emit_change();
    }

    /// Clear all selections
    pub fn clear(&self) {
        self.state.update(|s| s.clear());
        self.emit_change();
    }

    /// Check if item is selected
    pub fn is_selected(&self, item: &T) -> bool {
        self.state.with(|s| s.is_selected(item))
    }

    /// Get selection count
    pub fn count(&self) -> usize {
        self.state.with(|s| s.count())
    }

    /// Get all selected items
    pub fn get_all(&self) -> Vec<T> {
        self.state.with(|s| s.get_all())
    }

    fn emit_change(&self) {
        let state = self.state.get();
        if let Some(handler) = self.on_change.get() {
            handler.run(state);
        }
    }

    pub(crate) fn set_on_change(&self, callback: Option<Callback<SelectionState<T>>>) {
        self.on_change.set(callback);
    }
}

/// SelectionProvider - Provides selection context
#[component]
pub fn SelectionProvider<T>(
    children: Children,
    #[prop(default = SelectionMode::Multiple)] mode: SelectionMode,
    #[prop(optional)] on_change: Option<Callback<SelectionState<T>>>,
) -> impl IntoView
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static,
{
    let context = SelectionContext::new(mode);
    context.set_on_change(on_change);

    provide_context(context);

    view! { {children()} }
}

/// Hook to access selection context
pub fn use_selection<T>() -> SelectionContext<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash + Send + Sync + 'static,
{
    use_context::<SelectionContext<T>>()
        .expect("SelectionContext must be provided")
}

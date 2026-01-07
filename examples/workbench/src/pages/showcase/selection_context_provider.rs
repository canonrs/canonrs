use leptos::prelude::*;

/// SelectionContextValue - Context value with signals
#[derive(Clone, Copy)]
pub struct SelectionContextValue {
    pub selected_id: RwSignal<Option<String>>,
    pub node_type: RwSignal<Option<String>>,
    pub label: RwSignal<Option<String>>,
    pub metadata: RwSignal<Option<String>>,
}

impl SelectionContextValue {
    /// Update entire context at once
    pub fn update_selection(
        &self,
        id: Option<String>,
        node_type: Option<String>,
        label: Option<String>,
        metadata: Option<String>,
    ) {
        self.selected_id.set(id);
        self.node_type.set(node_type);
        self.label.set(label);
        self.metadata.set(metadata);
    }
    
    /// Clear selection
    pub fn clear(&self) {
        self.selected_id.set(None);
        self.node_type.set(None);
        self.label.set(None);
        self.metadata.set(None);
    }
    
    /// Check if has selection
    pub fn has_selection(&self) -> bool {
        self.selected_id.get().is_some()
    }
    
    /// Check if node is of type
    pub fn is_type(&self, type_name: &str) -> bool {
        self.node_type.get().as_deref() == Some(type_name)
    }
}

/// SelectionContextProvider - Provides selection context to children
/// 
/// **Pattern:** Top-level provider for tree selection state
/// **Usage:** Wrap your app/page in this provider
#[component]
pub fn SelectionContextProvider(children: Children) -> impl IntoView {
    let selected_id = RwSignal::new(None::<String>);
    let node_type = RwSignal::new(None::<String>);
    let label = RwSignal::new(None::<String>);
    let metadata = RwSignal::new(None::<String>);
    
    let context = SelectionContextValue {
        selected_id,
        node_type,
        label,
        metadata,
    };
    
    provide_context(context);
    
    view! { {children()} }
}

/// Hook to consume selection context
/// 
/// **Usage:**
/// ```rust
/// let ctx = use_selection_context();
/// let selected = ctx.selected_id.get();
/// ```
pub fn use_selection_context() -> SelectionContextValue {
    use_context::<SelectionContextValue>()
        .expect("SelectionContext not provided. Wrap component in <SelectionContextProvider>")
}

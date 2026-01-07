use leptos::prelude::*;

/// VirtualItemRenderer - Wrapper for rendering items in virtual list
///
/// **Type:** Pure Component (Type 1)
/// **Generic:** Works with any item type T
#[component]
pub fn VirtualItemRenderer<T, F, IV>(
    /// Item data
    item: T,
    
    /// Item index
    index: usize,
    
    /// Item height (for positioning)
    item_height: f64,
    
    /// Render function: (T, usize) -> View
    #[prop(into)]
    children: F,
) -> impl IntoView
where
    T: 'static,
    F: Fn(T, usize) -> IV + 'static,
    IV: IntoView,
{
    view! {
        <div style=format!("height: {}px", item_height)>
            {children(item, index)}
        </div>
    }
}
